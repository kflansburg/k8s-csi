//! # k8s-csi
//!
//! Automatically generated types, clients, and servers from Kubernetes CSI Protobuf definitions.
//!
//! ## Examples
//!
//! Connecting over TCP:
//!
//! ```no_run
//! # extern crate tokio;
//! use k8s_csi::v1_3_0::controller_client::ControllerClient;
//! use k8s_csi::v1_3_0::ListVolumesRequest;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut client = ControllerClient::connect("lttp://[::]:50051").await.expect("Could not create client.");
//!
//!     let request = tonic::Request::new(ListVolumesRequest {
//!         max_entries: 0,
//!         starting_token: "".to_string()
//!     });
//!     let response = client.list_volumes(request).await.expect("Request failed.");
//!     println!("{:?}", response);
//! }
//! ```
pub mod v1_3_0 {
    tonic::include_proto!("csi.v1");
}
