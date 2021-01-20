#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
pub mod google {
    pub mod pubsub {
        pub mod v1 {
            tonic::include_proto!("google.pubsub.v1");
        }
        pub mod v1beta2 {
            tonic::include_proto!("google.pubsub.v1beta2");
        }
    }
    //pub mod rpc {
    //    tonic::include_proto!("google.rpc");
    //}
    //pub mod r#type {
    //    tonic::include_proto!("google.r#type");
    //}
}

pub use google::pubsub::*;
pub use tonic;
