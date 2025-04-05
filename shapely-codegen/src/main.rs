use std::fmt::Write;

fn main() {
    // Check if current directory has a Cargo.toml with [workspace]
    let cargo_toml_path = std::env::current_dir().unwrap().join("Cargo.toml");
    let cargo_toml_content =
        std::fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    if !cargo_toml_content.contains("[workspace]") {
        panic!(
            "Cargo.toml does not contain [workspace] (you must run codegen from the workspace root)"
        );
    }

    let mut output = String::new();
    let _ = codegen_tuple_impls(&mut output);
    let path = "shapely-trait/src/impls/tuples_impls.rs";
    std::fs::write(path, output).expect("Failed to write file");

    // Run rustfmt on the generated file
    let status = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2024")
        .arg(path)
        .status()
        .expect("Failed to execute rustfmt");

    if !status.success() {
        eprintln!("rustfmt failed with exit code: {}", status);
    }
}

struct TupleGenerator {
    n: usize,
}

impl TupleGenerator {
    fn new(n: usize) -> Self {
        Self { n }
    }

    fn write_type_params<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        for i in 0..self.n {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "T{}", i)?;
        }
        Ok(())
    }

    fn write_where_clause<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        for i in 0..self.n {
            if i > 0 {
                write!(w, ", ")?;
            }
            write!(w, "T{}: Shapely", i)?;
        }
        Ok(())
    }

    fn write_tuple<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        if self.n == 1 {
            write!(w, "(T0,)")
        } else {
            write!(w, "(")?;
            self.write_type_params(w)?;
            write!(w, ")")
        }
    }

    fn write_dummy<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "(")?;
        for i in 0..self.n {
            if i > 0 {
                write!(w, ",")?;
            }
            write!(w, "T{}::DUMMY", i)?;
        }
        write!(w, ",)")
    }

    fn write_shape_list<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "&[")?;
        for i in 0..self.n {
            if i > 0 {
                write!(w, ",")?;
            }
            write!(w, "T{}::SHAPE", i)?;
        }
        write!(w, "]")
    }

    fn write_impl<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        // Start the implementation
        write!(w, "\nunsafe impl<")?;
        self.write_type_params(w)?;
        write!(w, "> Shapely for ")?;
        self.write_tuple(w)?;
        write!(w, " where ")?;
        self.write_where_clause(w)?;
        writeln!(w, " {{")?;

        // Write DUMMY constant
        write!(w, "    const DUMMY: Self = ")?;
        self.write_dummy(w)?;
        writeln!(w, ";")?;

        // Start SHAPE constant
        writeln!(w, "    const SHAPE: &'static Shape = &const {{")?;
        writeln!(w, "        use std::fmt;")?;
        writeln!(w)?;

        // Write type_name_fn
        self.write_type_name_fn(w)?;
        writeln!(w)?;

        // Write field_macro
        self.write_field_macro(w)?;
        writeln!(w)?;

        // Start Shape definition
        writeln!(w, "        Shape {{")?;
        write!(w, "            layout: Layout::new::<")?;
        self.write_tuple(w)?;
        writeln!(w, ">(),")?;

        // Start vtable
        writeln!(w, "            vtable: &ValueVTable {{")?;
        write!(w, "                type_name: type_name::<")?;
        self.write_type_params(w)?;
        writeln!(w, ">,")?;
        writeln!(w, "                display: None,")?;

        // Write debug implementation
        writeln!(w, "                debug: const {{")?;
        write!(w, "                    if Characteristic::Debug.all(")?;
        self.write_shape_list(w)?;
        writeln!(w, ") {{")?;
        writeln!(w, "                        Some(|value, f| {{")?;
        self.write_debug_fn_impl(w)?;
        writeln!(w, "                        }})")?;
        writeln!(w, "                    }} else {{")?;
        writeln!(w, "                        None")?;
        writeln!(w, "                    }}")?;
        writeln!(w, "                }},")?;

        // TODO: Add other vtable fields here
        writeln!(w, "                // ... (other vtable fields)")?;

        // Close vtable
        writeln!(w, "            }},")?;

        // Write def field
        writeln!(w, "            def: Def::Tuple(StructDef {{")?;
        write!(w, "                fields: ")?;
        self.write_fields(w)?;
        writeln!(w)?;
        writeln!(w, "            }}),")?;

        // Close everything
        writeln!(w, "        }}")?;
        writeln!(w, "    }};")?;
        writeln!(w, "}}")
    }

    fn write_type_name_fn<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "fn type_name<")?;
        self.write_type_params(w)?;
        writeln!(
            w,
            ">(f: &mut fmt::Formatter, opts: TypeNameOpts) -> fmt::Result"
        )?;
        write!(w, "where ")?;
        self.write_where_clause(w)?;
        writeln!(w)?;
        writeln!(w, "{{")?;
        writeln!(w, "    if let Some(opts) = opts.for_children() {{")?;
        writeln!(w, "        write!(f, \"(\")?;")?;

        for i in 0..self.n {
            if i > 0 {
                writeln!(w, "        write!(f, \", \")?;")?;
            }
            writeln!(w, "        (T{}::SHAPE.vtable.type_name)(f, opts)?;", i)?;
        }

        writeln!(w, "        write!(f, \")\")")?;
        writeln!(w, "    }} else {{")?;
        writeln!(w, "        write!(f, \"⋯\")")?;
        writeln!(w, "    }}")?;
        write!(w, "}}")?;
        Ok(())
    }

    fn write_field_macro<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        writeln!(w, "macro_rules! field {{")?;
        writeln!(w, "    ($idx:tt, $ty:ty) => {{")?;
        writeln!(w, "        Field {{")?;
        writeln!(w, "            name: stringify!($idx),")?;
        writeln!(w, "            shape: <$ty>::SHAPE,")?;
        write!(w, "            offset: std::mem::offset_of!(")?;
        self.write_tuple(w)?;
        writeln!(w, ", $idx),")?;
        writeln!(w, "            flags: FieldFlags::EMPTY,")?;
        writeln!(w, "        }}")?;
        writeln!(w, "    }}")?;
        write!(w, "}}")?;
        Ok(())
    }

    fn write_debug_fn_impl<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "let value = unsafe {{ value.as_ref::<")?;
        self.write_tuple(w)?;
        writeln!(w, ">() }};")?;
        writeln!(w, "write!(f, \"(\")?;")?;

        for i in 0..self.n {
            if i > 0 {
                writeln!(w, "write!(f, \", \")?;")?;
            }
            writeln!(
                w,
                "unsafe {{ (T{}::SHAPE.vtable.debug.unwrap_unchecked())(OpaqueConst::from_ref(&value.{}), f) }}?;",
                i, i
            )?;
        }

        write!(w, "write!(f, \")\")")?;
        Ok(())
    }

    fn write_fields<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "&const {{ [ ")?;

        for i in 0..self.n {
            if i > 0 {
                write!(w, ",")?;
            }
            write!(w, "field!({}, T{})", i, i)?;
        }

        write!(w, " ] }}")?;
        Ok(())
    }
}

fn codegen_tuple_impls(mut w: impl Write) -> std::fmt::Result {
    writeln!(w, "use std::alloc::Layout;")?;
    writeln!(w)?;
    writeln!(
        w,
        "use crate::{{Field, FieldFlags, Def, StructDef, Shape, Shapely, TypeNameOpts, ValueVTable, Characteristic, OpaqueConst}};"
    )?;

    // Generate implementations for tuples of size 1 to 12
    for n in 1..=12 {
        let generator = TupleGenerator::new(n);
        generator.write_impl(&mut w)?;
    }
    Ok(())
}
