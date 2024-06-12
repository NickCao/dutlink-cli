use std::{time::Duration, u8, usize};

use dutlink_cli::pb::{
    dutlink_service_server::{DutlinkService, DutlinkServiceServer},
    ConfigGetRequest, ConfigGetResponse, ConfigSetRequest, PinRequest, PowerRequest, ReadRequest,
    ReadResponse, StorageRequest,
};
use rusb::{request_type, Context, DeviceHandle, Direction, Recipient, RequestType, UsbContext};
use tonic::{transport::Server, Request, Response, Status};

struct Dutlink<const I: u8> {
    dev: DeviceHandle<Context>,
}

impl<const I: u8> Dutlink<I> {
    fn new() -> rusb::Result<Self> {
        let ctx = rusb::Context::new()?;
        let dev = ctx
            .open_device_with_vid_pid(0x2B23, 0x1012)
            .ok_or(rusb::Error::NoDevice)?;
        dev.claim_interface(I)?;
        Ok(Self { dev })
    }
    fn write_control(&self, request: u8, value: u16, buf: &[u8]) -> rusb::Result<usize> {
        self.dev.write_control(
            request_type(Direction::Out, RequestType::Vendor, Recipient::Interface),
            request,
            value,
            I.into(),
            buf,
            Duration::from_secs(1),
        )
    }
    fn read_control(&self, request: u8, value: u16, buf: &mut [u8]) -> rusb::Result<usize> {
        self.dev.read_control(
            request_type(Direction::In, RequestType::Vendor, Recipient::Interface),
            request,
            value,
            I.into(),
            buf,
            Duration::from_secs(1),
        )
    }
    fn refresh(&self) -> rusb::Result<usize> {
        self.write_control(0x00, 0, &[])
    }
}

#[tonic::async_trait]
impl<const I: u8> DutlinkService for Dutlink<I> {
    async fn power(&self, req: Request<PowerRequest>) -> Result<Response<()>, Status> {
        self.write_control(0x01, req.into_inner().state.try_into().unwrap(), &[])
            .unwrap();
        Ok(Response::new(()))
    }
    async fn storage(&self, req: Request<StorageRequest>) -> Result<Response<()>, Status> {
        self.write_control(0x02, req.into_inner().state.try_into().unwrap(), &[])
            .unwrap();
        Ok(Response::new(()))
    }
    async fn config_set(&self, req: Request<ConfigSetRequest>) -> Result<Response<()>, Status> {
        let inner = req.into_inner();
        self.write_control(0x03, inner.key.try_into().unwrap(), inner.value.as_bytes())
            .unwrap();
        Ok(Response::new(()))
    }
    async fn config_get(
        &self,
        req: Request<ConfigGetRequest>,
    ) -> Result<Response<ConfigGetResponse>, Status> {
        self.refresh().unwrap();
        let mut buf = vec![0u8; 128];
        let len = self
            .read_control(0x03, req.into_inner().key.try_into().unwrap(), &mut buf)
            .unwrap();
        buf.truncate(len);
        Ok(Response::new(ConfigGetResponse {
            value: String::from_utf8(buf).unwrap(),
        }))
    }
    async fn read(&self, req: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        self.refresh().unwrap();
        let mut buf = vec![0u8; 128];
        let len = self
            .read_control(0x04, req.into_inner().key.try_into().unwrap(), &mut buf)
            .unwrap();
        buf.truncate(len);
        Ok(Response::new(ReadResponse {
            value: String::from_utf8(buf).unwrap(),
        }))
    }
    async fn pin(&self, req: Request<PinRequest>) -> Result<Response<()>, Status> {
        let inner = req.into_inner();
        self.write_control(
            0x05,
            inner.pin.try_into().unwrap(),
            &[inner.state.try_into().unwrap()],
        )
        .unwrap();
        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() {
    Server::builder()
        .add_service(DutlinkServiceServer::new(Dutlink::<3>::new().unwrap()))
        .serve("[::1]:9000".parse().unwrap())
        .await
        .unwrap();
}
