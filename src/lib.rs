pub mod resources;
pub mod client;

#[cfg(test)]
mod tests {
    use crate::resources::*;

    #[test]
    fn account_struct() {
        Account {
            id: 99281,
            app_id: App::None(false),
            api_key: "dfba97c2b61a882793f4897abe84b6a0".into(),
            signout: Link {
                resource: ResourceLink {
                    url: "account/signout".into(),
                    link: "https://api.shoplightspeed.com/us/account/signout.json".into(),
                }
            },
            permissions: Link {
                resource: ResourceLink {
                    url: "account/permissions".into(),
                    link: "https://api.shoplightspeed.com/us/account/permissions.json".into(),
                }
            },
            ratelimit: Link {
                resource: ResourceLink {
                    url: "account/ratelimit".into(),
                    link: "https://api.shoplightspeed.com/us/account/permissions.json".into(),
                }
            },
            metafields: Link {
                resource: ResourceLink {
                    url: "account/metafields".into(),
                    link: "https://api.shoplightspeed.com/us/account/metafields.json".into(),
                }
            },
        };
    }
}
