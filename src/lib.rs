use scrypto::prelude::*;

#[derive(NonFungibleData)]
pub struct IdentidadData {
    id: String,
    #[scrypto(mutable)]
    datos: HashMap<String, String>
}

#[derive(NonFungibleData)]
pub struct Data {
    data: String
}

blueprint! {
    struct Identity {
        auth: Vault,
        identidad: ResourceDef,
        admin: ResourceDef
    }

    impl Identity {
       
        pub fn new() -> (Component, Bucket) {

            let admin: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Admin")
                .initial_supply_fungible(1);

            let auth: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name", "Auth")
                .initial_supply_fungible(1);

            let identidad: ResourceDef = ResourceBuilder::new_non_fungible()
                .metadata("name", "Identidad")
                .flags(MINTABLE|INDIVIDUAL_METADATA_MUTABLE)
                .badge(auth.resource_def(), MAY_MINT|MAY_CHANGE_INDIVIDUAL_METADATA)
                .no_initial_supply();
            
            let comp = Self {
                auth: Vault::with_bucket(auth),
                identidad,
                admin: admin.resource_def()
            }
            .instantiate();

            (comp, admin)
        }

        #[auth(admin)]
        pub fn mint(&mut self, id: String) -> Bucket {
            let datos = HashMap::new();
            self.auth.authorize(|auth| {
                self.identidad.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), IdentidadData{ id: id, datos: datos } , auth)
            })
        }

        #[auth(admin)]
        pub fn add(&mut self, identidad: BucketRef, clave: String, valor: String) {
            let mut data_nf: IdentidadData = self.identidad.get_non_fungible_data(&identidad.get_non_fungible_key());

            data_nf.datos.insert(clave, valor);

            self.auth.authorize(|auth| {
                self.identidad.update_non_fungible_data(&identidad.get_non_fungible_key(), data_nf, auth);
            });
        }

        pub fn split(&mut self, identidad: BucketRef, clave: String) -> Bucket {
            let mut data_nf: IdentidadData = self.identidad.get_non_fungible_data(&identidad.get_non_fungible_key());
            let valor: String = data_nf.datos.get_mut(&clave).unwrap().to_string();

            self.auth.authorize(|auth| {
                self.identidad.mint_non_fungible(&NonFungibleKey::from(Uuid::generate()), Data{ data: valor } , auth)
            })
        }


    }
}