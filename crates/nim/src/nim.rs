use anyhow::anyhow;
use heck::{ToLowerCamelCase, ToPascalCase};
use wit_bindgen_core::wit_parser::{FlagsRepr, Resolve, Type, TypeDefKind, TypeId};

fn nim_type_name(resolve: &Resolve, ty: Type) -> anyhow::Result<String> {
    match ty {
        Type::Bool => Ok("bool".to_string()),
        Type::U8 => Ok("uint8".to_string()),
        Type::U16 => Ok("uint16".to_string()),
        Type::U32 => Ok("uint32".to_string()),
        Type::U64 => Ok("uint64".to_string()),
        Type::S8 => Ok("int8".to_string()),
        Type::S16 => Ok("int16".to_string()),
        Type::S32 => Ok("int32".to_string()),
        Type::S64 => Ok("int64".to_string()),
        Type::F32 => Ok("float32".to_string()),
        Type::F64 => Ok("float64".to_string()),
        Type::Char => Ok("cchar".to_string()),
        Type::String => Ok("string".to_string()),
        Type::ErrorContext => Ok("ErrorContext".to_string()),
        Type::Id(id) => {
            let def = resolve
                .types
                .get(id)
                .ok_or(anyhow!("Type not found in provided resolve"))?;
            let name = def
                .name
                .as_ref()
                .ok_or(anyhow!("Type {id:?} has no name"))?;
            Ok(name.to_pascal_case())
        }
    }
}

pub struct Record {
    from: *const Resolve,
    id: TypeId,
}

impl Record {
    pub fn new(resolve: &Resolve, id: TypeId) -> anyhow::Result<Self> {
        if let Some(TypeDefKind::Record(_)) = resolve.types.get(id).map(|ty| &ty.kind) {
            Ok(Self { from: resolve, id })
        } else {
            Err(anyhow!("Type {id:?} is not a record"))
        }
    }

    pub fn to_string(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            return Err(anyhow!("Record is from a different resolve"));
        }

        let record_def = &resolve.types[self.id];
        let name = record_def
            .name
            .as_ref()
            .ok_or(anyhow!("Record {:?} has no name", self.id))?;
        let mut output = format!("type {name} {{.exportc.}} = object");

        let fields = if let TypeDefKind::Record(record) = &record_def.kind {
            &record.fields
        } else {
            return Err(anyhow!("Type {:?} is not a record", self.id));
        };

        for field in fields {
            let field_name = field.name.as_str().to_lower_camel_case();
            let field_type_name = nim_type_name(resolve, field.ty)?;

            if let Some(docs) = &field.docs.contents {
                output.push_str(docs);
            }
            output.push_str(&format!("\n  {field_name}: {field_type_name}",));
        }

        Ok(output)
    }
}

pub struct Resource {
    from: *const Resolve,
    id: TypeId,
}

impl Resource {
    pub fn new(resolve: &Resolve, id: TypeId) -> anyhow::Result<Self> {
        if let Some(TypeDefKind::Resource) = resolve.types.get(id).map(|ty| &ty.kind) {
            Ok(Self { from: resolve, id })
        } else {
            Err(anyhow!("Type {id:?} is not a resource"))
        }
    }

    pub fn to_string(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            Err(anyhow!("Resource is from a different resolve"))
        } else {
            let name = resolve.types[self.id].name.as_ref().unwrap();
            Ok(format!("type {name} = distinct int32"))
        }
    }
}

pub struct Handle {
    from: *const Resolve,
    id: TypeId,
}

impl Handle {
    pub fn new(resolve: &Resolve, id: TypeId) -> anyhow::Result<Self> {
        if let Some(TypeDefKind::Handle(_)) = resolve.types.get(id).map(|ty| &ty.kind) {
            Ok(Self { from: resolve, id })
        } else {
            Err(anyhow!("Type {id:?} is not a handle"))
        }
    }

    pub fn to_string(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            Err(anyhow!("Handle is from a different resolve"))
        } else {
            let name = resolve.types[self.id].name.as_ref().unwrap();
            Ok(format!("type {name} = distinct int32"))
        }
    }
}

pub struct Flags {
    from: *const Resolve,
    id: TypeId,
}

impl Flags {
    pub fn new(resolve: &Resolve, id: TypeId) -> anyhow::Result<Self> {
        if let Some(TypeDefKind::Flags(_)) = resolve.types.get(id).map(|ty| &ty.kind) {
            Ok(Self { from: resolve, id })
        } else {
            Err(anyhow!("Type {id:?} is not a flags"))
        }
    }

    fn discriminant_type(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            return Err(anyhow!("Flags is from a different resolve"));
        }

        let flags_def = &resolve.types[self.id];
        if let TypeDefKind::Flags(flags) = &flags_def.kind {
            let repr_ty = match flags.repr() {
                FlagsRepr::U8 => Type::U8,
                FlagsRepr::U16 => Type::U16,
                FlagsRepr::U32(1) => Type::U32,
                FlagsRepr::U32(2) => Type::U64,
                FlagsRepr::U32(n) => todo!("Generate type to represent flags with U32*{n} options"),
            };
            nim_type_name(resolve, repr_ty)
        } else {
            Err(anyhow!("Type {:?} is not flags", self.id))
        }
    }

    pub fn to_string(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            return Err(anyhow!("Flags is from a different resolve"));
        }

        let flags_def = &resolve.types[self.id];
        let name = flags_def
            .name
            .as_ref()
            .ok_or(anyhow!("Flags {:?} has no name", self.id))?;
        let mut output = format!("type {name} {{.exportc.}} = object");

        let flags = if let TypeDefKind::Flags(flags) = &flags_def.kind {
            &flags.flags
        } else {
            return Err(anyhow!("Type {:?} is not a record", self.id));
        };

        for flag in flags {
            let flag_name = flag.name.as_str().to_lower_camel_case();

            if let Some(docs) = &flag.docs.contents {
                output.push_str(docs);
            }
            output.push_str(&format!("\n  {flag_name} {{.bitsize:1.}}: bool",));
        }

        Ok(output)
    }
}

pub struct Tuple {
    from: *const Resolve,
    id: TypeId,
}

impl Tuple {
    pub fn new(resolve: &Resolve, id: TypeId) -> anyhow::Result<Self> {
        if let Some(TypeDefKind::Tuple(_)) = resolve.types.get(id).map(|ty| &ty.kind) {
            Ok(Self { from: resolve, id })
        } else {
            Err(anyhow!("Type {id:?} is not a tuple"))
        }
    }

    pub fn to_string(&self, resolve: &Resolve) -> anyhow::Result<String> {
        if self.from != resolve {
            return Err(anyhow!("Tuple is from a different resolve"));
        }

        let tuple_def = &resolve.types[self.id];
        let name = tuple_def.name.as_ref();

        let tuple = if let TypeDefKind::Tuple(tuple) = &tuple_def.kind {
            tuple
        } else {
            return Err(anyhow!("Type {:?} is not a tuple", self.id));
        };

        let mut output = name.map(|name| format!("type {name} {{.exportc.}} = tuple[")).unwrap_or("tuple[".to_string());
        for (index, &ty) in tuple.types.iter().enumerate() {
            let ty_name = nim_type_name(resolve, ty)?;
            output.push_str(&format!("`{index}`: {ty_name},"));
        }
        output.push(']');

        Ok(output)
    };
}
