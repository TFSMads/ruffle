use crate::avm2::activation::Activation;
use crate::avm2::api_version::ApiVersion;
use crate::avm2::class::Class;
use crate::avm2::domain::Domain;
use crate::avm2::object::{ClassObject, Object, ScriptObject, TObject};
use crate::avm2::scope::{Scope, ScopeChain};
use crate::avm2::script::Script;
use crate::avm2::Avm2;
use crate::avm2::Error;
use crate::avm2::Namespace;
use crate::avm2::QName;
use crate::string::AvmString;
use crate::tag_utils::{self, ControlFlow, SwfMovie, SwfSlice, SwfStream};
use gc_arena::{Collect, GcCell, Mutation};
use std::sync::Arc;
use swf::TagCode;

mod __ruffle__;
mod array;
mod avmplus;
mod boolean;
mod class;
mod date;
mod error;
pub mod flash;
mod function;
mod global_scope;
mod int;
mod json;
mod math;
mod namespace;
mod number;
mod object;
mod q_name;
mod reg_exp;
mod string;
mod toplevel;
mod r#uint;
mod vector;
mod void;
mod xml;
mod xml_list;

pub use toplevel::decode_uri;
pub use toplevel::decode_uri_component;
pub use toplevel::encode_uri;
pub use toplevel::encode_uri_component;
pub use toplevel::escape;
pub use toplevel::is_finite;
pub use toplevel::is_na_n;
pub use toplevel::is_xml_name;
pub use toplevel::parse_float;
pub use toplevel::parse_int;
pub use toplevel::trace;
pub use toplevel::unescape;

/// This structure represents all system builtin classes.
#[derive(Clone, Collect)]
#[collect(no_drop)]
pub struct SystemClasses<'gc> {
    pub object: ClassObject<'gc>,
    pub function: ClassObject<'gc>,
    pub class: ClassObject<'gc>,
    pub global: ClassObject<'gc>,
    pub string: ClassObject<'gc>,
    pub boolean: ClassObject<'gc>,
    pub number: ClassObject<'gc>,
    pub int: ClassObject<'gc>,
    pub uint: ClassObject<'gc>,
    pub void: ClassObject<'gc>,
    pub namespace: ClassObject<'gc>,
    pub array: ClassObject<'gc>,
    pub movieclip: ClassObject<'gc>,
    pub framelabel: ClassObject<'gc>,
    pub scene: ClassObject<'gc>,
    pub application_domain: ClassObject<'gc>,
    pub event: ClassObject<'gc>,
    pub fullscreenevent: ClassObject<'gc>,
    pub video: ClassObject<'gc>,
    pub xml: ClassObject<'gc>,
    pub xml_list: ClassObject<'gc>,
    pub display_object: ClassObject<'gc>,
    pub shape: ClassObject<'gc>,
    pub textfield: ClassObject<'gc>,
    pub textformat: ClassObject<'gc>,
    pub graphics: ClassObject<'gc>,
    pub igraphicsdata: ClassObject<'gc>,
    pub graphicsbitmapfill: ClassObject<'gc>,
    pub graphicsendfill: ClassObject<'gc>,
    pub graphicsgradientfill: ClassObject<'gc>,
    pub graphicspath: ClassObject<'gc>,
    pub graphicstrianglepath: ClassObject<'gc>,
    pub graphicssolidfill: ClassObject<'gc>,
    pub graphicsshaderfill: ClassObject<'gc>,
    pub graphicsstroke: ClassObject<'gc>,
    pub loader: ClassObject<'gc>,
    pub loaderinfo: ClassObject<'gc>,
    pub bytearray: ClassObject<'gc>,
    pub stage: ClassObject<'gc>,
    pub sprite: ClassObject<'gc>,
    pub simplebutton: ClassObject<'gc>,
    pub regexp: ClassObject<'gc>,
    // the generic Vector class, useless until you .apply() type arg onto it
    pub generic_vector: ClassObject<'gc>,
    // Vector.<*>, NOT Vector.<Object>. Used as base class for new Vector<T>.
    pub object_vector: ClassObject<'gc>,
    pub soundtransform: ClassObject<'gc>,
    pub soundchannel: ClassObject<'gc>,
    pub bitmap: ClassObject<'gc>,
    pub bitmapdata: ClassObject<'gc>,
    pub date: ClassObject<'gc>,
    pub qname: ClassObject<'gc>,
    pub mouseevent: ClassObject<'gc>,
    pub progressevent: ClassObject<'gc>,
    pub httpstatusevent: ClassObject<'gc>,
    pub textevent: ClassObject<'gc>,
    pub errorevent: ClassObject<'gc>,
    pub ioerrorevent: ClassObject<'gc>,
    pub securityerrorevent: ClassObject<'gc>,
    pub transform: ClassObject<'gc>,
    pub colortransform: ClassObject<'gc>,
    pub matrix: ClassObject<'gc>,
    pub illegaloperationerror: ClassObject<'gc>,
    pub eventdispatcher: ClassObject<'gc>,
    pub rectangle: ClassObject<'gc>,
    pub keyboardevent: ClassObject<'gc>,
    pub point: ClassObject<'gc>,
    pub evalerror: ClassObject<'gc>,
    pub rangeerror: ClassObject<'gc>,
    pub referenceerror: ClassObject<'gc>,
    pub argumenterror: ClassObject<'gc>,
    pub syntaxerror: ClassObject<'gc>,
    pub typeerror: ClassObject<'gc>,
    pub verifyerror: ClassObject<'gc>,
    pub ioerror: ClassObject<'gc>,
    pub eoferror: ClassObject<'gc>,
    pub urierror: ClassObject<'gc>,
    pub securityerror: ClassObject<'gc>,
    pub error: ClassObject<'gc>,
    pub uncaughterrorevents: ClassObject<'gc>,
    pub statictext: ClassObject<'gc>,
    pub textlinemetrics: ClassObject<'gc>,
    pub stage3d: ClassObject<'gc>,
    pub context3d: ClassObject<'gc>,
    pub indexbuffer3d: ClassObject<'gc>,
    pub vertexbuffer3d: ClassObject<'gc>,
    pub program3d: ClassObject<'gc>,
    pub urlvariables: ClassObject<'gc>,
    pub bevelfilter: ClassObject<'gc>,
    pub bitmapfilter: ClassObject<'gc>,
    pub blurfilter: ClassObject<'gc>,
    pub colormatrixfilter: ClassObject<'gc>,
    pub convolutionfilter: ClassObject<'gc>,
    pub displacementmapfilter: ClassObject<'gc>,
    pub dropshadowfilter: ClassObject<'gc>,
    pub glowfilter: ClassObject<'gc>,
    pub gradientbevelfilter: ClassObject<'gc>,
    pub gradientglowfilter: ClassObject<'gc>,
    pub texture: ClassObject<'gc>,
    pub cubetexture: ClassObject<'gc>,
    pub rectangletexture: ClassObject<'gc>,
    pub morphshape: ClassObject<'gc>,
    pub shader: ClassObject<'gc>,
    pub shaderinput: ClassObject<'gc>,
    pub shaderparameter: ClassObject<'gc>,
    pub netstatusevent: ClassObject<'gc>,
    pub shaderfilter: ClassObject<'gc>,
    pub statusevent: ClassObject<'gc>,
    pub contextmenuevent: ClassObject<'gc>,
    pub filereference: ClassObject<'gc>,
    pub filefilter: ClassObject<'gc>,
    pub font: ClassObject<'gc>,
    pub textline: ClassObject<'gc>,
    pub sampledataevent: ClassObject<'gc>,
    pub avm1movie: ClassObject<'gc>,
    pub focusevent: ClassObject<'gc>,
}

impl<'gc> SystemClasses<'gc> {
    /// Construct a minimal set of system classes necessary for bootstrapping
    /// player globals.
    ///
    /// All other system classes aside from the three given here will be set to
    /// the empty object also handed to this function. It is the caller's
    /// responsibility to instantiate each class and replace the empty object
    /// with that.
    fn new(
        object: ClassObject<'gc>,
        function: ClassObject<'gc>,
        class: ClassObject<'gc>,
        global: ClassObject<'gc>,
    ) -> Self {
        SystemClasses {
            object,
            function,
            class,
            global,
            // temporary initialization
            string: object,
            boolean: object,
            number: object,
            int: object,
            uint: object,
            void: object,
            namespace: object,
            array: object,
            movieclip: object,
            framelabel: object,
            scene: object,
            application_domain: object,
            event: object,
            fullscreenevent: object,
            video: object,
            xml: object,
            xml_list: object,
            display_object: object,
            shape: object,
            textfield: object,
            textformat: object,
            graphics: object,
            igraphicsdata: object,
            graphicsbitmapfill: object,
            graphicsendfill: object,
            graphicsgradientfill: object,
            graphicspath: object,
            graphicstrianglepath: object,
            graphicssolidfill: object,
            graphicsshaderfill: object,
            graphicsstroke: object,
            loader: object,
            loaderinfo: object,
            bytearray: object,
            stage: object,
            sprite: object,
            simplebutton: object,
            regexp: object,
            generic_vector: object,
            object_vector: object,
            soundtransform: object,
            soundchannel: object,
            bitmap: object,
            bitmapdata: object,
            date: object,
            qname: object,
            mouseevent: object,
            progressevent: object,
            httpstatusevent: object,
            textevent: object,
            errorevent: object,
            ioerrorevent: object,
            securityerrorevent: object,
            transform: object,
            colortransform: object,
            matrix: object,
            illegaloperationerror: object,
            eventdispatcher: object,
            rectangle: object,
            keyboardevent: object,
            point: object,
            evalerror: object,
            rangeerror: object,
            referenceerror: object,
            argumenterror: object,
            syntaxerror: object,
            typeerror: object,
            verifyerror: object,
            ioerror: object,
            eoferror: object,
            urierror: object,
            securityerror: object,
            error: object,
            uncaughterrorevents: object,
            statictext: object,
            textlinemetrics: object,
            stage3d: object,
            context3d: object,
            indexbuffer3d: object,
            vertexbuffer3d: object,
            program3d: object,
            urlvariables: object,
            bevelfilter: object,
            bitmapfilter: object,
            blurfilter: object,
            colormatrixfilter: object,
            convolutionfilter: object,
            displacementmapfilter: object,
            dropshadowfilter: object,
            glowfilter: object,
            gradientbevelfilter: object,
            gradientglowfilter: object,
            texture: object,
            cubetexture: object,
            rectangletexture: object,
            morphshape: object,
            shader: object,
            shaderinput: object,
            shaderparameter: object,
            netstatusevent: object,
            shaderfilter: object,
            statusevent: object,
            contextmenuevent: object,
            filereference: object,
            filefilter: object,
            font: object,
            textline: object,
            sampledataevent: object,
            avm1movie: object,
            focusevent: object,
        }
    }
}

/// Looks up a function defined in the script domain, and defines it on the global object.
///
/// This expects the looked-up value to be a function.
fn define_fn_on_global<'gc>(
    activation: &mut Activation<'_, 'gc>,
    package: impl Into<AvmString<'gc>>,
    name: &'static str,
    script: Script<'gc>,
) {
    let (_, global, domain) = script.init();
    let qname = QName::new(
        Namespace::package(
            package,
            ApiVersion::AllVersions,
            &mut activation.borrow_gc(),
        ),
        name,
    );
    let func = domain
        .get_defined_value(activation, qname)
        .expect("Function being defined on global should be defined in domain!");

    global.install_const_late(
        activation.context.gc_context,
        qname,
        func,
        activation.avm2().classes().function,
    );
}

/// Add a fully-formed class object builtin to the global scope.
///
/// This allows the caller to pre-populate the class's prototype with dynamic
/// properties, if necessary.
fn dynamic_class<'gc>(
    mc: &Mutation<'gc>,
    class_object: ClassObject<'gc>,
    script: Script<'gc>,
    // The `ClassObject` of the `Class` class
    class_class: ClassObject<'gc>,
) {
    let (_, global, mut domain) = script.init();
    let class = class_object.inner_class_definition();
    let name = class.read().name();

    global.install_const_late(mc, name, class_object.into(), class_class);
    domain.export_definition(name, script, mc)
}

/// Add a class builtin to the global scope.
///
/// This function returns the class object and class prototype as a class, which
/// may be stored in `SystemClasses`
fn class<'gc>(
    class_def: GcCell<'gc, Class<'gc>>,
    script: Script<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<ClassObject<'gc>, Error<'gc>> {
    let mc = activation.context.gc_context;
    let (_, global, mut domain) = script.init();

    let class_read = class_def.read();
    let super_class = if let Some(sc_name) = class_read.super_class_name() {
        let super_class: Result<Object<'gc>, Error<'gc>> = activation
            .resolve_definition(sc_name)
            .ok()
            .and_then(|v| v)
            .and_then(|v| v.as_object())
            .ok_or_else(|| {
                format!(
                    "Could not resolve superclass {} when defining global class {}",
                    sc_name.to_qualified_name(mc),
                    class_read.name().to_qualified_name(mc)
                )
                .into()
            });
        let super_class = super_class?
            .as_class_object()
            .ok_or_else(|| Error::from("Base class of a global class is not a class"))?;

        Some(super_class)
    } else {
        None
    };

    let class_name = class_read.name();
    drop(class_read);

    let class_object = ClassObject::from_class(activation, class_def, super_class)?;
    global.install_const_late(
        mc,
        class_name,
        class_object.into(),
        activation.avm2().classes().class,
    );
    domain.export_definition(class_name, script, mc);
    domain.export_class(class_name, class_def, mc);
    Ok(class_object)
}

fn vector_class<'gc>(
    param_class: Option<ClassObject<'gc>>,
    legacy_name: &'static str,
    script: Script<'gc>,
    activation: &mut Activation<'_, 'gc>,
) -> Result<ClassObject<'gc>, Error<'gc>> {
    let mc = activation.context.gc_context;
    let (_, global, mut domain) = script.init();

    let cls = param_class.map(|c| c.inner_class_definition());
    let vector_cls = class(
        vector::create_builtin_class(activation, cls),
        script,
        activation,
    )?;
    vector_cls.set_param(mc, Some(param_class));

    let generic_vector = activation.avm2().classes().generic_vector;
    generic_vector.add_application(mc, param_class, vector_cls);
    let generic_cls = generic_vector.inner_class_definition();
    generic_cls
        .write(mc)
        .add_application(cls, vector_cls.inner_class_definition());

    let legacy_name = QName::new(activation.avm2().vector_internal_namespace, legacy_name);
    global.install_const_late(
        mc,
        legacy_name,
        vector_cls.into(),
        activation.avm2().classes().class,
    );
    domain.export_definition(legacy_name, script, mc);
    Ok(vector_cls)
}

macro_rules! avm2_system_class {
    ($field:ident, $activation:ident, $class:expr, $script:expr) => {
        let class_object = class($class, $script, $activation)?;

        let sc = $activation.avm2().system_classes.as_mut().unwrap();
        sc.$field = class_object;
    };
}

/// Initialize the player global domain.
///
/// This should be called only once, to construct the global scope of the
/// player. It will return a list of prototypes it has created, which should be
/// stored on the AVM. All relevant declarations will also be attached to the
/// given domain.
pub fn load_player_globals<'gc>(
    activation: &mut Activation<'_, 'gc>,
    domain: Domain<'gc>,
) -> Result<(), Error<'gc>> {
    let mc = activation.context.gc_context;

    let globals = ScriptObject::custom_object(mc, None, None);
    let gs = ScopeChain::new(domain).chain(mc, &[Scope::new(globals)]);
    let script = Script::empty_script(mc, globals, domain);

    // Set the outer scope of this activation to the global scope.
    activation.set_outer(gs);

    // public / root package
    //
    // This part of global initialization is very complicated, because
    // everything has to circularly reference everything else:
    //
    //  - Object is an instance of itself, as well as it's prototype
    //  - All other types are instances of Class, which is an instance of
    //    itself
    //  - Function's prototype is an instance of itself
    //  - All methods created by the above-mentioned classes are also instances
    //    of Function
    //  - All classes are put on Global's trait list, but Global needs
    //    to be initialized first, but you can't do that until Object/Class are ready.
    //
    // Hence, this ridiculously complicated dance of classdef, type allocation,
    // and partial initialization.
    let object_classdef = object::create_class(activation);
    let object_class = ClassObject::from_class_partial(activation, object_classdef, None)?;
    let object_proto = ScriptObject::custom_object(mc, Some(object_class), None);
    domain.export_class(object_classdef.read().name(), object_classdef, mc);

    let fn_classdef = function::create_class(activation);
    let fn_class = ClassObject::from_class_partial(activation, fn_classdef, Some(object_class))?;
    let fn_proto = ScriptObject::custom_object(mc, Some(fn_class), Some(object_proto));
    domain.export_class(fn_classdef.read().name(), fn_classdef, mc);

    let class_classdef = class::create_class(activation);
    let class_class =
        ClassObject::from_class_partial(activation, class_classdef, Some(object_class))?;
    let class_proto = ScriptObject::custom_object(mc, Some(object_class), Some(object_proto));
    domain.export_class(class_classdef.read().name(), class_classdef, mc);

    let global_classdef = global_scope::create_class(activation);
    let global_class =
        ClassObject::from_class_partial(activation, global_classdef, Some(object_class))?;
    let global_proto = ScriptObject::custom_object(mc, Some(object_class), Some(object_proto));
    domain.export_class(global_classdef.read().name(), global_classdef, mc);

    // Now to weave the Gordian knot...
    object_class.link_prototype(activation, object_proto)?;
    object_class.link_type(mc, class_proto, class_class);

    fn_class.link_prototype(activation, fn_proto)?;
    fn_class.link_type(mc, class_proto, class_class);

    class_class.link_prototype(activation, class_proto)?;
    class_class.link_type(mc, class_proto, class_class);

    global_class.link_prototype(activation, global_proto)?;
    global_class.link_type(mc, class_proto, class_class);

    // At this point, we need at least a partial set of system classes in
    // order to continue initializing the player. The rest of the classes
    // are set to a temporary class until we have a chance to initialize them.

    activation.context.avm2.system_classes = Some(SystemClasses::new(
        object_class,
        fn_class,
        class_class,
        global_class,
    ));

    // Our activation environment is now functional enough to finish
    // initializing the core class weave. We need to initialize superclasses
    // (e.g. `Object`) before subclasses, so that `into_finished_class` can
    // copy traits from the initialized superclass vtable.

    // First, initialize the instance vtable, starting with `Object`. This
    // ensures that properties defined in `Object` (e.g. `hasOwnProperty`)
    // get copied into the vtable of `Class`, `Function`, and `Global`.
    // (which are all subclasses of `Object`)
    object_class.init_instance_vtable(activation)?;
    class_class.init_instance_vtable(activation)?;
    fn_class.init_instance_vtable(activation)?;
    global_class.init_instance_vtable(activation)?;

    // Now, construct the `ClassObject`s, starting with `Class`. This ensures
    // that the `prototype` property of `Class` gets copied into the *class*
    // vtables for `Object`, `Function`, and `Global`.
    let class_class = class_class.into_finished_class(activation)?;
    let object_class = object_class.into_finished_class(activation)?;
    let fn_class = fn_class.into_finished_class(activation)?;
    let _global_class = global_class.into_finished_class(activation)?;

    globals.set_proto(mc, global_proto);
    globals.set_instance_of(mc, global_class);
    globals.fork_vtable(mc);

    activation.context.avm2.toplevel_global_object = Some(globals);

    // From this point, `globals` is safe to be modified

    dynamic_class(mc, object_class, script, class_class);
    dynamic_class(mc, fn_class, script, class_class);
    dynamic_class(mc, class_class, script, class_class);

    // After this point, it is safe to initialize any other classes.
    // Make sure to initialize superclasses *before* their subclasses!

    avm2_system_class!(string, activation, string::create_class(activation), script);
    avm2_system_class!(
        boolean,
        activation,
        boolean::create_class(activation),
        script
    );
    avm2_system_class!(number, activation, number::create_class(activation), script);
    avm2_system_class!(int, activation, int::create_class(activation), script);
    avm2_system_class!(uint, activation, uint::create_class(activation), script);
    avm2_system_class!(
        namespace,
        activation,
        namespace::create_class(activation),
        script
    );
    avm2_system_class!(array, activation, array::create_class(activation), script);

    // TODO: this should _not_ be exposed as a ClassObject, getDefinitionByName etc.
    // it should only be visible as an type for typecheck/cast purposes.
    avm2_system_class!(void, activation, void::create_class(activation), script);

    avm2_system_class!(
        generic_vector,
        activation,
        vector::create_generic_class(activation),
        script
    );

    vector_class(
        Some(activation.avm2().classes().int),
        "Vector$int",
        script,
        activation,
    )?;
    vector_class(
        Some(activation.avm2().classes().uint),
        "Vector$uint",
        script,
        activation,
    )?;
    vector_class(
        Some(activation.avm2().classes().number),
        "Vector$double",
        script,
        activation,
    )?;
    let object_vector = vector_class(None, "Vector$object", script, activation)?;
    activation
        .avm2()
        .system_classes
        .as_mut()
        .unwrap()
        .object_vector = object_vector;

    avm2_system_class!(date, activation, date::create_class(activation), script);

    // Inside this call, the macro `avm2_system_classes_playerglobal`
    // triggers classloading. Therefore, we run `load_playerglobal`
    // relatively late, so that it can access classes defined before
    // this call.
    load_playerglobal(activation, domain)?;

    // Except for `trace`, top-level builtin functions are defined
    // on the `global` object.
    define_fn_on_global(activation, "", "decodeURI", script);
    define_fn_on_global(activation, "", "decodeURIComponent", script);
    define_fn_on_global(activation, "", "encodeURI", script);
    define_fn_on_global(activation, "", "encodeURIComponent", script);
    define_fn_on_global(activation, "", "escape", script);
    define_fn_on_global(activation, "", "unescape", script);
    define_fn_on_global(activation, "", "isXMLName", script);
    define_fn_on_global(activation, "", "isFinite", script);
    define_fn_on_global(activation, "", "isNaN", script);
    define_fn_on_global(activation, "", "parseFloat", script);
    define_fn_on_global(activation, "", "parseInt", script);

    Ok(())
}

/// This file is built by 'core/build_playerglobal/'
/// See that tool, and 'core/src/avm2/globals/README.md', for more details
const PLAYERGLOBAL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/playerglobal.swf"));

mod native {
    include!(concat!(env!("OUT_DIR"), "/native_table.rs"));
}

/// Loads classes from our custom 'playerglobal' (which are written in ActionScript)
/// into the environment. See 'core/src/avm2/globals/README.md' for more information
fn load_playerglobal<'gc>(
    activation: &mut Activation<'_, 'gc>,
    domain: Domain<'gc>,
) -> Result<(), Error<'gc>> {
    activation.avm2().native_method_table = native::NATIVE_METHOD_TABLE;
    activation.avm2().native_instance_allocator_table = native::NATIVE_INSTANCE_ALLOCATOR_TABLE;
    activation.avm2().native_instance_init_table = native::NATIVE_INSTANCE_INIT_TABLE;
    activation.avm2().native_call_handler_table = native::NATIVE_CALL_HANDLER_TABLE;

    let movie = Arc::new(
        SwfMovie::from_data(PLAYERGLOBAL, "file:///".into(), None)
            .expect("playerglobal.swf should be valid"),
    );

    let slice = SwfSlice::from(movie.clone());

    let mut reader = slice.read_from(0);

    let tag_callback = |reader: &mut SwfStream<'_>, tag_code, _tag_len| {
        if tag_code == TagCode::DoAbc2 {
            let do_abc = reader
                .read_do_abc_2()
                .expect("playerglobal.swf should be valid");
            Avm2::do_abc(
                &mut activation.context,
                do_abc.data,
                None,
                do_abc.flags,
                domain,
                movie.clone(),
            )
            .expect("playerglobal.swf should be valid");
        } else if tag_code != TagCode::End {
            panic!("playerglobal should only contain `DoAbc2` tag - found tag {tag_code:?}")
        }
        Ok(ControlFlow::Continue)
    };

    let _ = tag_utils::decode_tags(&mut reader, tag_callback);
    macro_rules! avm2_system_classes_playerglobal {
        ($activation:expr, $script:expr, [$(($package:expr, $class_name:expr, $field:ident)),* $(,)?]) => {
            let activation = $activation;
            $(
                // Lookup with the highest version, so we we see all defined classes here
                let ns = Namespace::package($package, ApiVersion::VM_INTERNAL, &mut activation.borrow_gc());
                let name = QName::new(ns, $class_name);
                let class_object = activation.domain().get_defined_value(activation, name).unwrap_or_else(|e| panic!("Failed to lookup {name:?}: {e:?}"));
                let class_object = class_object.as_object().unwrap().as_class_object().unwrap();
                let sc = activation.avm2().system_classes.as_mut().unwrap();
                sc.$field = class_object;
            )*
        }
    }

    // This acts the same way as 'avm2_system_class', but for classes
    // declared in 'playerglobal'. Classes are declared as ("package", "class", field_name),
    // and are stored in 'avm2().system_classes'
    avm2_system_classes_playerglobal!(
        &mut *activation,
        script,
        [
            ("", "Error", error),
            ("", "ArgumentError", argumenterror),
            ("", "QName", qname),
            ("", "EvalError", evalerror),
            ("", "RangeError", rangeerror),
            ("", "RegExp", regexp),
            ("", "ReferenceError", referenceerror),
            ("", "SecurityError", securityerror),
            ("", "SyntaxError", syntaxerror),
            ("", "TypeError", typeerror),
            ("", "URIError", urierror),
            ("", "VerifyError", verifyerror),
            ("", "XML", xml),
            ("", "XMLList", xml_list),
            ("flash.display", "AVM1Movie", avm1movie),
            ("flash.display", "Bitmap", bitmap),
            ("flash.display", "BitmapData", bitmapdata),
            ("flash.display", "Scene", scene),
            ("flash.display", "FrameLabel", framelabel),
            ("flash.display", "IGraphicsData", igraphicsdata),
            ("flash.display", "GraphicsBitmapFill", graphicsbitmapfill),
            ("flash.display", "GraphicsEndFill", graphicsendfill),
            (
                "flash.display",
                "GraphicsGradientFill",
                graphicsgradientfill
            ),
            ("flash.display", "GraphicsPath", graphicspath),
            (
                "flash.display",
                "GraphicsTrianglePath",
                graphicstrianglepath
            ),
            ("flash.display", "GraphicsSolidFill", graphicssolidfill),
            ("flash.display", "GraphicsStroke", graphicsstroke),
            ("flash.display", "Graphics", graphics),
            ("flash.display", "Loader", loader),
            ("flash.display", "LoaderInfo", loaderinfo),
            ("flash.display", "MorphShape", morphshape),
            ("flash.display", "MovieClip", movieclip),
            ("flash.display", "ShaderInput", shaderinput),
            ("flash.display", "ShaderParameter", shaderparameter),
            ("flash.display", "Shape", shape),
            ("flash.display", "SimpleButton", simplebutton),
            ("flash.display", "Sprite", sprite),
            ("flash.display", "Stage", stage),
            ("flash.display", "Stage3D", stage3d),
            ("flash.display3D", "Context3D", context3d),
            ("flash.display3D", "IndexBuffer3D", indexbuffer3d),
            ("flash.display3D", "Program3D", program3d),
            ("flash.display3D.textures", "CubeTexture", cubetexture),
            ("flash.display3D.textures", "Texture", texture),
            (
                "flash.display3D.textures",
                "RectangleTexture",
                rectangletexture
            ),
            ("flash.display3D", "VertexBuffer3D", vertexbuffer3d),
            (
                "flash.errors",
                "IllegalOperationError",
                illegaloperationerror
            ),
            ("flash.errors", "IOError", ioerror),
            ("flash.errors", "EOFError", eoferror),
            ("flash.events", "Event", event),
            ("flash.events", "EventDispatcher", eventdispatcher),
            ("flash.events", "TextEvent", textevent),
            ("flash.events", "ErrorEvent", errorevent),
            ("flash.events", "KeyboardEvent", keyboardevent),
            ("flash.events", "ProgressEvent", progressevent),
            ("flash.events", "HTTPStatusEvent", httpstatusevent),
            ("flash.events", "SecurityErrorEvent", securityerrorevent),
            ("flash.events", "IOErrorEvent", ioerrorevent),
            ("flash.events", "MouseEvent", mouseevent),
            ("flash.events", "FullScreenEvent", fullscreenevent),
            ("flash.events", "UncaughtErrorEvents", uncaughterrorevents),
            ("flash.events", "NetStatusEvent", netstatusevent),
            ("flash.events", "StatusEvent", statusevent),
            ("flash.events", "ContextMenuEvent", contextmenuevent),
            ("flash.events", "FocusEvent", focusevent),
            ("flash.geom", "Matrix", matrix),
            ("flash.geom", "Point", point),
            ("flash.geom", "Rectangle", rectangle),
            ("flash.geom", "Transform", transform),
            ("flash.geom", "ColorTransform", colortransform),
            ("flash.media", "SoundChannel", soundchannel),
            ("flash.media", "SoundTransform", soundtransform),
            ("flash.media", "Video", video),
            ("flash.net", "URLVariables", urlvariables),
            ("flash.net", "FileReference", filereference),
            ("flash.net", "FileFilter", filefilter),
            ("flash.utils", "ByteArray", bytearray),
            ("flash.system", "ApplicationDomain", application_domain),
            ("flash.text", "Font", font),
            ("flash.text", "StaticText", statictext),
            ("flash.text", "TextFormat", textformat),
            ("flash.text", "TextField", textfield),
            ("flash.text", "TextLineMetrics", textlinemetrics),
            ("flash.text.engine", "TextLine", textline),
            ("flash.filters", "BevelFilter", bevelfilter),
            ("flash.filters", "BitmapFilter", bitmapfilter),
            ("flash.filters", "BlurFilter", blurfilter),
            ("flash.filters", "ColorMatrixFilter", colormatrixfilter),
            ("flash.filters", "ConvolutionFilter", convolutionfilter),
            (
                "flash.filters",
                "DisplacementMapFilter",
                displacementmapfilter
            ),
            ("flash.filters", "DropShadowFilter", dropshadowfilter),
            ("flash.filters", "GlowFilter", glowfilter),
            ("flash.filters", "GradientBevelFilter", gradientbevelfilter),
            ("flash.filters", "GradientGlowFilter", gradientglowfilter),
            ("flash.filters", "ShaderFilter", shaderfilter),
            ("flash.events", "SampleDataEvent", sampledataevent),
        ]
    );

    // Domain memory must be initialized after playerglobals is loaded because it relies on ByteArray.
    domain.init_default_domain_memory(activation)?;
    activation
        .avm2()
        .stage_domain()
        .init_default_domain_memory(activation)?;
    Ok(())
}
