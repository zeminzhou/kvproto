// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KEYSPACE_LOAD_KEYSPACE: ::grpcio::Method<super::keyspacepb::LoadKeyspaceRequest, super::keyspacepb::LoadKeyspaceResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/keyspacepb.Keyspace/LoadKeyspace",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KEYSPACE_WATCH_KEYSPACES: ::grpcio::Method<super::keyspacepb::WatchKeyspacesRequest, super::keyspacepb::WatchKeyspacesResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/keyspacepb.Keyspace/WatchKeyspaces",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct KeyspaceClient {
    client: ::grpcio::Client,
}

impl KeyspaceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KeyspaceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn load_keyspace_opt(&self, req: &super::keyspacepb::LoadKeyspaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::keyspacepb::LoadKeyspaceResponse> {
        self.client.unary_call(&METHOD_KEYSPACE_LOAD_KEYSPACE, req, opt)
    }

    pub fn load_keyspace(&self, req: &super::keyspacepb::LoadKeyspaceRequest) -> ::grpcio::Result<super::keyspacepb::LoadKeyspaceResponse> {
        self.load_keyspace_opt(req, ::grpcio::CallOption::default())
    }

    pub fn load_keyspace_async_opt(&self, req: &super::keyspacepb::LoadKeyspaceRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keyspacepb::LoadKeyspaceResponse>> {
        self.client.unary_call_async(&METHOD_KEYSPACE_LOAD_KEYSPACE, req, opt)
    }

    pub fn load_keyspace_async(&self, req: &super::keyspacepb::LoadKeyspaceRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::keyspacepb::LoadKeyspaceResponse>> {
        self.load_keyspace_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn watch_keyspaces_opt(&self, req: &super::keyspacepb::WatchKeyspacesRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::keyspacepb::WatchKeyspacesResponse>> {
        self.client.server_streaming(&METHOD_KEYSPACE_WATCH_KEYSPACES, req, opt)
    }

    pub fn watch_keyspaces(&self, req: &super::keyspacepb::WatchKeyspacesRequest) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::keyspacepb::WatchKeyspacesResponse>> {
        self.watch_keyspaces_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Output = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Keyspace {
    fn load_keyspace(&mut self, ctx: ::grpcio::RpcContext, req: super::keyspacepb::LoadKeyspaceRequest, sink: ::grpcio::UnarySink<super::keyspacepb::LoadKeyspaceResponse>);
    fn watch_keyspaces(&mut self, ctx: ::grpcio::RpcContext, req: super::keyspacepb::WatchKeyspacesRequest, sink: ::grpcio::ServerStreamingSink<super::keyspacepb::WatchKeyspacesResponse>);
}

pub fn create_keyspace<S: Keyspace + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KEYSPACE_LOAD_KEYSPACE, move |ctx, req, resp| {
        instance.load_keyspace(ctx, req, resp)
    });
    let mut instance = s;
    builder = builder.add_server_streaming_handler(&METHOD_KEYSPACE_WATCH_KEYSPACES, move |ctx, req, resp| {
        instance.watch_keyspaces(ctx, req, resp)
    });
    builder.build()
}
