use std::{cmp::max, time::Duration};

use pulsar_binary_protocol_spec::{
    client_handler::{ReadCommandError, WriteCommandError},
    command::{Command, CommandWithParsed},
    frame::{FrameParseOutput, FrameParser, FrameRenderer},
    types::{ConsumerIdBuilder, ProducerIdBuilder, RequestIdBuilder},
};

use super::{AsyncRead, AsyncReadWithTimeoutExt, AsyncWrite, AsyncWriteExt};

#[derive(Default, Debug, Clone)]
pub struct AsyncConnectionConfig {
    read_timeout: Option<Duration>,
}
impl AsyncConnectionConfig {
    pub fn set_read_timeout(&mut self, dur: Duration) -> &mut Self {
        self.read_timeout = Some(dur);
        self
    }
    fn get_read_timeout(&self) -> Duration {
        self.read_timeout
            .unwrap_or_else(|| Duration::from_millis(100))
    }
}

pub struct AsyncConnection<S> {
    stream: S,
    config: AsyncConnectionConfig,
    frame_renderer: FrameRenderer,
    frame_renderer_buf: Vec<u8>,
    frame_parser: FrameParser,
    frame_parser_buf: Vec<u8>,
    frame_parser_buf_n_read: usize,
    frame_parser_buf_n_parsed: usize,
    pub(crate) request_id_builder: RequestIdBuilder,
    pub(crate) producer_id_builder: ProducerIdBuilder,
    pub(crate) consumer_id_builder: ConsumerIdBuilder,
}

impl<S> AsyncConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(stream: S, config: impl Into<Option<AsyncConnectionConfig>>) -> Self {
        Self {
            stream,
            config: config.into().unwrap_or_default(),
            frame_renderer: FrameRenderer::default(),
            frame_renderer_buf: Vec::with_capacity(5 * 1024 * 1024),
            frame_parser: FrameParser::default(),
            frame_parser_buf: vec![0; 5 * 1024 * 1024],
            frame_parser_buf_n_read: 0,
            frame_parser_buf_n_parsed: 0,
            request_id_builder: Default::default(),
            producer_id_builder: Default::default(),
            consumer_id_builder: Default::default(),
        }
    }

    pub(crate) fn get_mut_frame_renderer(&mut self) -> &mut FrameRenderer {
        &mut self.frame_renderer
    }

    pub(crate) fn get_mut_frame_parser(&mut self) -> &mut FrameParser {
        &mut self.frame_parser
    }

    pub(crate) async fn write_command<C>(&mut self, command: C) -> Result<(), WriteCommandError>
    where
        C: Into<Command>,
    {
        self.frame_renderer
            .render(command, &mut self.frame_renderer_buf)?;
        self.stream.write_all(&self.frame_renderer_buf[..]).await?;

        self.frame_renderer_buf.clear();

        Ok(())
    }

    pub(crate) async fn try_read_commands(
        &mut self,
        max_size: impl Into<Option<usize>>,
    ) -> Result<Option<Vec<CommandWithParsed>>, ReadCommandError> {
        let n = self
            .stream
            .read_with_timeout(
                &mut self.frame_parser_buf[self.frame_parser_buf_n_read..],
                self.config.get_read_timeout(),
            )
            .await?;
        self.frame_parser_buf_n_read += n;
        if n == 0 {
            return Ok(None);
        }

        let max_size = max_size.into();

        let mut commands = vec![];
        loop {
            match self.frame_parser.parse(
                &self.frame_parser_buf
                    [self.frame_parser_buf_n_parsed..self.frame_parser_buf_n_read],
            )? {
                FrameParseOutput::Completed(n, command) => {
                    self.frame_parser_buf_n_parsed += n;

                    let frame_parser_buf_n_parsed = self.frame_parser_buf_n_parsed;
                    self.frame_parser_buf.rotate_left(frame_parser_buf_n_parsed);
                    self.frame_parser_buf_n_read -= frame_parser_buf_n_parsed;
                    self.frame_parser_buf_n_parsed = 0;

                    commands.push(command);

                    if let Some(max_size) = max_size {
                        if max(max_size, 1) >= commands.len() {
                            return Ok(Some(commands));
                        }
                    }

                    continue;
                }
                FrameParseOutput::Partial(n) => {
                    self.frame_parser_buf_n_parsed += n;

                    if let Some(total_size) = self.frame_parser.get_total_size() {
                        if self.frame_parser_buf.len() < total_size as usize {
                            self.frame_parser_buf.resize(total_size as usize, 0)
                        }
                    }

                    break;
                }
            }
        }

        if commands.is_empty() {
            Ok(None)
        } else {
            Ok(Some(commands))
        }
    }
}
