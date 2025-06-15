mod nim;

use anyhow::{anyhow, Ok};
use wit_bindgen_core::{
    wit_parser::{self, InterfaceId, Resolve, TypeDefKind, TypeId, WorldKey},
    WorldGenerator,
};

#[derive(Default, Debug, Clone)]
#[cfg_attr(feature = "clap", derive(clap::Parser))]
pub struct Opts {}
impl Opts {
    pub fn build(&self) -> Box<dyn WorldGenerator> {
        Box::new(Nim::default())
    }
}

#[derive(Default)]
struct Nim {
    records: Vec<nim::Record>,
    resources: Vec<nim::Resource>,
    handles: Vec<nim::Handle>,
    flags: Vec<nim::Flags>,
    tuples: Vec<nim::Tuple>,
    variants: Vec<nim::Variant>,
    enums: Vec<nim::Enum>,
    options: Vec<nim::Option>,
}

impl Nim {
    fn new() -> Self {
        Self::default()
    }

    fn dump_resolve(resolve: &Resolve) -> anyhow::Result<()> {
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .open("resolve.txt")?;

        for (id, def) in &resolve.types {
            writeln!(&mut file, "{id:?}\n\t{def:#?}\n")?;
        }

        Ok(())
    }

    fn add_record(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let record = nim::Record::new(resolve, id)?;
        self.records.push(record);
        Ok(())
    }

    fn add_resource(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let resource = nim::Resource::new(resolve, id)?;
        self.resources.push(resource);
        Ok(())
    }

    fn add_handle(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let handle = nim::Handle::new(resolve, id)?;
        self.handles.push(handle);
        Ok(())
    }

    fn add_flags(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let flags = nim::Flags::new(resolve, id)?;
        self.flags.push(flags);
        Ok(())
    }

    fn add_tuple(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let tuple = nim::Tuple::new(resolve, id)?;
        self.tuples.push(tuple);
        Ok(())
    }

    fn add_variant(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let variant = nim::Variant::new(resolve, id)?;
        self.variants.push(variant);
        Ok(())
    }

    fn add_enum(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let r#enum = nim::Enum::new(resolve, id)?;
        self.enums.push(r#enum);
        Ok(())
    }

    fn add_option(&mut self, resolve: &Resolve, id: TypeId) -> anyhow::Result<()> {
        let option = nim::Option::new(resolve, id)?;
        self.options.push(option);
        Ok(())
    }
}

impl WorldGenerator for Nim {
    fn import_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        iface: InterfaceId,
        files: &mut wit_bindgen_core::Files,
    ) -> anyhow::Result<()> {
        Self::dump_resolve(resolve)?;

        for (id, def) in &resolve.types {
            match &def.kind {
                TypeDefKind::Record(_) => self.add_record(resolve, id)?,
                TypeDefKind::Resource => self.add_resource(resolve, id)?,
                TypeDefKind::Handle(_) => self.add_handle(resolve, id)?,
                TypeDefKind::Flags(_) => self.add_flags(resolve, id)?,
                TypeDefKind::Tuple(_) => self.add_tuple(resolve, id)?,
                TypeDefKind::Variant(_) => self.add_variant(resolve, id)?,
                TypeDefKind::Enum(_) => self.add_enum(resolve, id)?,
                TypeDefKind::Option(_) => self.add_option(resolve, id)?,
                TypeDefKind::Result(_) => todo!(),
                TypeDefKind::List(_) => todo!(),
                TypeDefKind::FixedSizeList(_, _) => todo!(),
                TypeDefKind::Future(_) => todo!(),
                TypeDefKind::Stream(_) => todo!(),
                TypeDefKind::Type(_) => todo!(),
                TypeDefKind::Unknown => {}
            }
        }

        Err(anyhow!("import_interface: Unimplemented"))
    }

    fn export_interface(
        &mut self,
        resolve: &Resolve,
        name: &WorldKey,
        iface: InterfaceId,
        files: &mut wit_bindgen_core::Files,
    ) -> anyhow::Result<()> {
        let a = files.iter().collect::<Vec<_>>();
        dbg!(resolve, name, iface, a);

        Err(anyhow!("export_interface: Unimplemented"))
    }

    fn import_funcs(
        &mut self,
        resolve: &Resolve,
        world: wit_parser::WorldId,
        funcs: &[(&str, &wit_parser::Function)],
        files: &mut wit_bindgen_core::Files,
    ) {
        let f = funcs.iter().collect::<Vec<_>>();
        dbg!(resolve, world, funcs, f);
    }

    fn export_funcs(
        &mut self,
        resolve: &Resolve,
        world: wit_parser::WorldId,
        funcs: &[(&str, &wit_parser::Function)],
        files: &mut wit_bindgen_core::Files,
    ) -> anyhow::Result<()> {
        let f = funcs.iter().collect::<Vec<_>>();
        dbg!(resolve, world, funcs, f);

        Err(anyhow!("export_funcs: Unimplemented"))
    }

    fn import_types(
        &mut self,
        resolve: &Resolve,
        world: wit_parser::WorldId,
        types: &[(&str, wit_parser::TypeId)],
        files: &mut wit_bindgen_core::Files,
    ) {
        let t = types.iter().collect::<Vec<_>>();
        dbg!(resolve, world, types, t);
    }

    fn finish(
        &mut self,
        resolve: &Resolve,
        world: wit_parser::WorldId,
        files: &mut wit_bindgen_core::Files,
    ) -> anyhow::Result<()> {
        let a = files.iter().collect::<Vec<_>>();
        dbg!(resolve, world, a);

        Err(anyhow!("finish: Unimplemented"))
    }
}
