pub mod lol_api {

    pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("lol-grpc-models");

    pub mod google {
        pub mod protobuf {
            use serde::{Deserialize, Serialize};
            tonic::include_proto!("google.protobuf");
        }
    }
    pub mod com {
        pub mod mckernant1 {
            pub mod lol {
                pub mod leagues {
                    use serde::{Deserialize, Serialize};
                    tonic::include_proto!("com.mckernant1.lol.leagues");
                }
                pub mod players {
                    use serde::{Deserialize, Serialize};
                    tonic::include_proto!("com.mckernant1.lol.players");
                }
                pub mod matches {
                    use serde::{Deserialize, Serialize};
                    tonic::include_proto!("com.mckernant1.lol.matches");
                }
                pub mod teams {
                    use serde::{Deserialize, Serialize};
                    tonic::include_proto!("com.mckernant1.lol.teams");
                }
                pub mod tournament {
                    use serde::{Deserialize, Serialize};
                    tonic::include_proto!("com.mckernant1.lol.tournament");
                }
            }
        }
    }
}
