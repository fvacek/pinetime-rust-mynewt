input: ItemForeignMod {
    attrs: [],
    abi: Abi {
        extern_token: Extern,
        name: Some(
            LitStr {
                token: Literal { lit: Str_(C), suffix: None, span: Span { lo: BytePos(300127), hi: BytePos(300130), ctxt: #0 } },
            },
        ),
    },
    brace_token: Brace,
    items: [
        Fn(
            ForeignItemFn {
                attrs: [],
                vis: Public(
                    VisPublic {
                        pub_token: Pub,
                    },
                ),
                ident: Ident {
                    ident: "os_task_init",
                    span: #0 bytes(300148..300160),
                },
                decl: FnDecl {
                    fn_token: Fn,
                    generics: Generics {
                        lt_token: None,
                        params: [],
                        gt_token: None,
                        where_clause: None,
                    },
                    paren_token: Paren,
                    inputs: [
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg1",
                                            span: #0 bytes(300174..300178),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Ptr(
                                    TypePtr {
                                        star_token: Star,
                                        const_token: None,
                                        mutability: Some(
                                            Mut,
                                        ),
                                        elem: Path(
                                            TypePath {
                                                qself: None,
                                                path: Path {
                                                    leading_colon: None,
                                                    segments: [
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "os_task",
                                                                span: #0 bytes(300185..300192),
                                                            },
                                                            arguments: None,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg2",
                                            span: #0 bytes(300206..300210),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Ptr(
                                    TypePtr {
                                        star_token: Star,
                                        const_token: Some(
                                            Const,
                                        ),
                                        mutability: None,
                                        elem: Path(
                                            TypePath {
                                                qself: None,
                                                path: Path {
                                                    leading_colon: Some(
                                                        Colon2,
                                                    ),
                                                    segments: [
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "cty",
                                                                span: #0 bytes(300221..300224),
                                                            },
                                                            arguments: None,
                                                        },
                                                        Colon2,
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "c_char",
                                                                span: #0 bytes(300226..300232),
                                                            },
                                                            arguments: None,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg3",
                                            span: #0 bytes(300246..300250),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "os_task_func_t",
                                                        span: #0 bytes(300252..300266),
                                                    },
                                                    arguments: None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg4",
                                            span: #0 bytes(300280..300284),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Ptr(
                                    TypePtr {
                                        star_token: Star,
                                        const_token: None,
                                        mutability: Some(
                                            Mut,
                                        ),
                                        elem: Path(
                                            TypePath {
                                                qself: None,
                                                path: Path {
                                                    leading_colon: Some(
                                                        Colon2,
                                                    ),
                                                    segments: [
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "cty",
                                                                span: #0 bytes(300293..300296),
                                                            },
                                                            arguments: None,
                                                        },
                                                        Colon2,
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "c_void",
                                                                span: #0 bytes(300298..300304),
                                                            },
                                                            arguments: None,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg5",
                                            span: #0 bytes(300318..300322),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "u8",
                                                        span: #0 bytes(300324..300326),
                                                    },
                                                    arguments: None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg6",
                                            span: #0 bytes(300340..300344),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "os_time_t",
                                                        span: #0 bytes(300346..300355),
                                                    },
                                                    arguments: None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg7",
                                            span: #0 bytes(300369..300373),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Ptr(
                                    TypePtr {
                                        star_token: Star,
                                        const_token: None,
                                        mutability: Some(
                                            Mut,
                                        ),
                                        elem: Path(
                                            TypePath {
                                                qself: None,
                                                path: Path {
                                                    leading_colon: None,
                                                    segments: [
                                                        PathSegment {
                                                            ident: Ident {
                                                                ident: "os_stack_t",
                                                                span: #0 bytes(300380..300390),
                                                            },
                                                            arguments: None,
                                                        },
                                                    ],
                                                },
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                        Comma,
                        Captured(
                            ArgCaptured {
                                pat: Ident(
                                    PatIdent {
                                        by_ref: None,
                                        mutability: None,
                                        ident: Ident {
                                            ident: "arg8",
                                            span: #0 bytes(300404..300408),
                                        },
                                        subpat: None,
                                    },
                                ),
                                colon_token: Colon,
                                ty: Path(
                                    TypePath {
                                        qself: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "u16",
                                                        span: #0 bytes(300410..300413),
                                                    },
                                                    arguments: None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            },
                        ),
                        Comma,
                    ],
                    variadic: None,
                    output: Type(
                        RArrow,
                        Path(
                            TypePath {
                                qself: None,
                                path: Path {
                                    leading_colon: Some(
                                        Colon2,
                                    ),
                                    segments: [
                                        PathSegment {
                                            ident: Ident {
                                                ident: "cty",
                                                span: #0 bytes(300430..300433),
                                            },
                                            arguments: None,
                                        },
                                        Colon2,
                                        PathSegment {
                                            ident: Ident {
                                                ident: "c_int",
                                                span: #0 bytes(300435..300440),
                                            },
                                            arguments: None,
                                        },
                                    ],
                                },
                            },
                        ),
                    ),
                },
                semi_token: Semi,
            },
        ),
    ],
}
#![feature(prelude_import)]
#![no_std]
//!  Sensor app that reads sensor data from a temperature sensor and sends the sensor data to a CoAP server or Collector Node.
//!  Note that we are using a patched version of apps/my_sensor_app/src/vsscanf.c that
//!  fixes ESP8266 response parsing bugs.  The patched file must be present in that location.
//!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust/apps/my_sensor_app/OLDsrc/main.c`

#![no_std]
//  Don't link with standard Rust library, which is not compatible with embedded systems
#![feature(trace_macros)]
//  Allow macro tracing: `trace_macros!(true)`
#![feature(concat_idents)]
//  Allow `concat_idents!()` macro used in `coap!()` macro
#![feature(const_transmute)]
//  Allow `transmute` for initialising Mynewt structs
#![feature(proc_macro_hygiene)]
//  Allow Procedural Macros like `run!()`
#![feature(custom_attribute)]
#[prelude_import]
use ::core::prelude::v1::*;
#[macro_use]
extern crate core as core;
#[macro_use]
extern crate compiler_builtins as compiler_builtins;
//  Allow Custom Attributes like `#[safe_wrap]`

extern crate cortex_m;
//  Declare the external library `cortex_m`
extern crate mynewt_macros;
//  Declare the Mynewt Macros library, because it exports Procedural Macros

//  Suppress warnings of unused constants and vars
//  Allow type names to have non-camel case
//  Allow globals to have lowercase letters
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
mod mynewt {
    //  Declare `mynewt/mod.rs` as Rust module `mynewt`

    //  Suppress warnings of unused constants and vars
    //  Declare `base.rs` as Rust module `base`

    //  Declare `listen_sensor.rs` as Rust module `listen_sensor`
    //  Declare `send_coap.rs` as Rust module `send_coap`

    //  Import `PanicInfo` type which is used by `panic()` below
    //  Import cortex_m assembly function to inject breakpoint
    //  Import Mynewt OS API
    //  Import `base.rs` for common declarations

    //  Don't mangle the name "main"
    //  Declare extern "C" because it will be called by Mynewt
    //  Init Mynewt system.


    //  Start the Network Task in the background.  The Network Task prepares the ESP8266 or nRF24L01 transceiver for
    //  sending CoAP messages.  We connect the ESP8266 to the WiFi access point and register
    //  the ESP8266/nRF24L01 driver as the network transport for CoAP.  Also perform WiFi Geolocation if it is enabled.

    //  Starting polling the temperature sensor every 10 seconds in the background.  
    //  After polling the sensor, call the listener function to send the sensor data to the CoAP server or Collector Node.
    //  If this is the Collector Node, we shall wait for sensor data from the Sensor Nodes and transmit to the CoAP server.

    //  Main event loop
    //  Loop forever...
    //  Process events...
    //  From default event queue.
    //  Never comes here.

    //  Display the filename and line number to the Semihosting Console.
    //  TODO: Print in decimal not hex. Allow more than 255 lines.
    //  Pause in the debugger.
    //  Loop forever so that device won't restart.
    //! Mynewt API for Rust. Contains Rust bindings for Mynewt API for C, generated by `bindgen`.
    //! Also includes safe versions of Mynewt APIs created specially for Rust.
    #[macro_use]
    pub mod macros {
        //!  Mynewt Macros for Rust. Note that macros defined locally should be called with `$crate::`, like `$crate::parse`.
        //!  This works with Rust compiler versions 1.30 and later.  See https://doc.rust-lang.org/stable/edition-guide/rust-2018/macros/macro-changes.html
        //!  To see the expanded macros: `cargo rustc -- -Z unstable-options --pretty expanded`
        ///  Return a const struct that has all fields set to 0. Used for initialising static mutable structs like `os_task`.
        ///  `fill_zero!(os_task)` expands to
        ///  ```
        /// unsafe { 
        ///	::core::mem::transmute::
        ///	<
        ///	  [
        ///		u8; 
        ///		::core::mem::size_of::<os_task>()
        ///	  ], 
        ///	  os_task
        ///	>
        ///	(
        ///	  [
        ///		0; 
        ///		::core::mem::size_of::<os_task>()
        ///	  ]
        ///	) 
        /// }
        ///  ```
        #[macro_export]
        macro_rules! fill_zero(( $ type : ident ) => {
                               unsafe {
                               :: core :: mem :: transmute :: < [
                               u8 ; :: core :: mem :: size_of :: < $ type > (
                               ) ] , $ type > (
                               [
                               0 ; :: core :: mem :: size_of :: < $ type > (
                               ) ] ) } } ;);
        ///  Macro to compose a CoAP payload with JSON or CBOR encoding.
        ///  First parameter is `@none`, `@json` or `@cbor`, to indicate
        ///  no encoding (testing), JSON encoding or CBOR encoding.
        ///  Second parameter is the JSON message to be transmitted.
        ///  Adapted from the `json!()` macro: https://docs.serde.rs/src/serde_json/macros.rs.html
        #[macro_export]
        macro_rules! coap(( @ none $ ( $ tokens : tt ) + ) => {
                          $ crate :: parse ! ( @ none $ ( $ tokens ) + ) } ; (
                          @ json $ ( $ tokens : tt ) + ) => {
                          $ crate :: parse ! ( @ json $ ( $ tokens ) + ) } ; (
                          @ cbor $ ( $ tokens : tt ) + ) => {
                          $ crate :: parse ! ( @ cbor $ ( $ tokens ) + ) } ;);
        ///  Parse the JSON code in the parameter and compose the CoAP payload.
        ///  This macro takes these parameters:
        ///  - __Encoding__: `@json`, `@cbor` or `@none`
        ///  - __State__: Current parsing state (`@object`, `@array` or omitted)
        ///  - __Context__: JSON or CBOR parsing context (`JsonContext` or `CborContext`)
        ///  - __Remaining tokens__ to be parsed
        ///  - __Remaining tokens__ again, for error display
        #[macro_export]
        macro_rules! parse((
                           @ $ enc : ident @ object $ object : ident (  ) (  )
                           (  ) ) => {  } ; (
                           @ none @ object $ object : ident [
                           $ ( $ key : tt ) + ] ( $ value : expr ) , $ (
                           $ rest : tt ) * ) => {
                           d ! (
                           TODO : add key : $ ( $ key ) + , value : $ value ,
                           to object : $ object ) ; $ crate :: parse ! (
                           @ none @ object $ object (  ) ( $ ( $ rest ) * ) (
                           $ ( $ rest ) * ) ) ; } ; (
                           @ $ enc : ident @ object $ object : ident [
                           $ ( $ key : tt ) + ] ( $ value : expr ) , $ (
                           $ rest : tt ) * ) => {
                           d ! (
                           add1 key : $ ( $ key ) + value : $ value to object
                           : $ object ) ; $ crate :: coap_item_str ! (
                           @ $ enc $ object , $ ( $ key ) + , $ value ) ;
                           "--------------------" ; $ crate :: parse ! (
                           @ $ enc @ object $ object (  ) ( $ ( $ rest ) * ) (
                           $ ( $ rest ) * ) ) ; } ; (
                           @ $ enc : ident @ object $ object : ident [
                           $ ( $ key : tt ) + ] ( $ value : expr ) $
                           unexpected : tt $ ( $ rest : tt ) * ) => {
                           unexpected_token ! ( $ unexpected ) ; } ; (
                           @ $ enc : ident @ object $ object : ident [
                           $ ( $ key : tt ) + ] ( $ value : expr ) ) => {
                           d ! (
                           TODO : add2 key : $ ( $ key ) + value : $ value to
                           object : $ object ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) ( : null $ ( $ rest : tt ) * )
                           $ copy : tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc null ) ) $ ( $ rest )
                           * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) ( : true $ ( $ rest : tt ) * )
                           $ copy : tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc true ) ) $ ( $ rest )
                           * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) ( : false $ ( $ rest : tt ) *
                           ) $ copy : tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc false ) ) $ ( $ rest )
                           * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) (
                           : [ $ ( $ array : tt ) * ] $ ( $ rest : tt ) * ) $
                           copy : tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc [ $ ( $ array ) * ] )
                           ) $ ( $ rest ) * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) (
                           : { $ ( $ map : tt ) * } $ ( $ rest : tt ) * ) $
                           copy : tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc { $ ( $ map ) * } ) )
                           $ ( $ rest ) * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) (
                           : $ value : expr , $ ( $ rest : tt ) * ) $ copy :
                           tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc $ value ) ) , $ (
                           $ rest ) * ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) ( : $ value : expr ) $ copy :
                           tt ) => {
                           $ crate :: parse ! (
                           @ $ enc @ object $ object [ $ ( $ key ) + ] (
                           $ crate :: parse ! ( @ $ enc $ value ) ) ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) ( : ) $ copy : tt ) => {
                           $ crate :: parse ! (  ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) + ) (  ) $ copy : tt ) => {
                           $ crate :: parse ! (  ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (  ) (
                           : $ ( $ rest : tt ) * ) (
                           $ colon : tt $ ( $ copy : tt ) * ) ) => {
                           $ crate :: unexpected_token ! ( $ colon ) ; } ; (
                           @ none @ object $ object : ident (
                           $ ( $ key : tt ) * ) ( , $ ( $ rest : tt ) * ) (
                           $ comma : tt $ ( $ copy : tt ) * ) ) => {
                           d ! (
                           TODO : extract key , value from _sensor_value : $ (
                           $ key ) * and add to _object : $ object ) ;
                           "--------------------" ; $ crate :: parse ! (
                           @ none @ object $ object (  ) ( $ ( $ rest ) * ) (
                           $ ( $ rest ) * ) ) ; } ; (
                           @ json @ object $ object : ident (
                           $ ( $ key : tt ) * ) ( , $ ( $ rest : tt ) * ) (
                           $ comma : tt $ ( $ copy : tt ) * ) ) => {
                           "--------------------" ; $ crate ::
                           coap_item_int_val ! (
                           @ json $ object , $ ( $ key ) * ) ;
                           "--------------------" ; $ crate :: parse ! (
                           @ json @ object $ object (  ) ( $ ( $ rest ) * ) (
                           $ ( $ rest ) * ) ) ; } ; (
                           @ cbor @ object $ object : ident (
                           $ ( $ key : tt ) * ) ( , $ ( $ rest : tt ) * ) (
                           $ comma : tt $ ( $ copy : tt ) * ) ) => {
                           "--------------------" ; $ crate ::
                           coap_set_int_val ! (
                           @ cbor $ object , $ ( $ key ) * ) ;
                           "--------------------" ; $ crate :: parse ! (
                           @ cbor @ object $ object (  ) ( $ ( $ rest ) * ) (
                           $ ( $ rest ) * ) ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (  ) (
                           ( $ key : expr ) : $ ( $ rest : tt ) * ) $ copy :
                           tt ) => {
                           d ! ( got (  ) ) ; $ crate :: parse ! (
                           @ $ enc @ object $ object ( $ key ) (
                           : $ ( $ rest ) * ) ( : $ ( $ rest ) * ) ) ; } ; (
                           @ $ enc : ident @ object $ object : ident (
                           $ ( $ key : tt ) * ) (
                           $ tt : tt $ ( $ rest : tt ) * ) $ copy : tt ) => {
                           $ crate :: nx ! (
                           ( $ ( $ key ) * ) , ( $ tt ) , ( $ ( $ rest ) * ) )
                           ; $ crate :: parse ! (
                           @ $ enc @ object $ object ( $ ( $ key ) * $ tt ) (
                           $ ( $ rest ) * ) ( $ ( $ rest ) * ) ) ; } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] ) => { parse_vector ! [ $ ( $ elems , ) * ] } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr ) , *
                           ] ) => { parse_vector ! [ $ ( $ elems ) , * ] } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] null $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! ( @ $ enc null
                           ) ] $ ( $ rest ) * ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] true $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! ( @ $ enc true
                           ) ] $ ( $ rest ) * ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] false $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! (
                           @ $ enc false ) ] $ ( $ rest ) * ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] [ $ ( $ array : tt ) * ] $ ( $ rest : tt ) * ) =>
                           {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! (
                           @ $ enc [ $ ( $ array ) * ] ) ] $ ( $ rest ) * ) }
                           ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] { $ ( $ map : tt ) * } $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! (
                           @ $ enc { $ ( $ map ) * } ) ] $ ( $ rest ) * ) } ;
                           (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] $ next : expr , $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! (
                           @ $ enc $ next ) , ] $ ( $ rest ) * ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr , ) *
                           ] $ last : expr ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [
                           $ ( $ elems , ) * $ crate :: parse ! (
                           @ $ enc $ last ) ] ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr ) , *
                           ] , $ ( $ rest : tt ) * ) => {
                           $ crate :: parse ! (
                           @ $ enc @ array [ $ ( $ elems , ) * ] $ ( $ rest )
                           * ) } ; (
                           @ $ enc : ident @ array [ $ ( $ elems : expr ) , *
                           ] $ unexpected : tt $ ( $ rest : tt ) * ) => {
                           $ crate :: unexpected_token ! ( $ unexpected ) } ;
                           ( @ $ enc : ident null ) => {
                           { d ! ( TODO : null ) ; "null" } } ; (
                           @ $ enc : ident true ) => {
                           { d ! ( true ) ; "true" } } ; (
                           @ $ enc : ident false ) => {
                           { d ! ( false ) ; "false" } } ; (
                           @ $ enc : ident [  ] ) => {
                           { d ! ( [ TODO ] ) ; "[ TODO ]" } } ; (
                           @ $ enc : ident [ $ ( $ tt : tt ) + ] ) => {
                           {
                           d ! ( begin array ) ; _array = $ crate :: parse ! (
                           @ $ enc @ array [  ] $ ( $ tt ) + ) ; d ! (
                           end array ) ; "[ TODO ]" } } ; (
                           @ $ enc : ident {  } ) => {
                           { d ! ( { TODO } ) ; "{ TODO }" } } ; (
                           @ none { $ ( $ tt : tt ) + } ) => {
                           {
                           d ! ( begin none root ) ; let root = "root" ; $
                           crate :: parse ! (
                           @ none @ object root (  ) ( $ ( $ tt ) + ) (
                           $ ( $ tt ) + ) ) ; d ! ( end none root ) ; d ! (
                           return none root to caller ) ; root } } ; (
                           @ json { $ ( $ tt : tt ) + } ) => {
                           {
                           d ! ( begin json root ) ; $ crate :: coap_root ! (
                           @ json JSON_CONTEXT {
                           $ crate :: coap_array ! (
                           @ json JSON_CONTEXT , values , {
                           $ crate :: parse ! (
                           @ json @ object JSON_CONTEXT (  ) ( $ ( $ tt ) + )
                           ( $ ( $ tt ) + ) ) ; } ) ; } ) ; d ! (
                           end json root ) ; (  ) } } ; (
                           @ cbor { $ ( $ tt : tt ) + } ) => {
                           {
                           d ! ( begin cbor root ) ; $ crate :: coap_root ! (
                           @ cbor JSON_CONTEXT {
                           $ crate :: parse ! (
                           @ cbor @ object JSON_CONTEXT (  ) ( $ ( $ tt ) + )
                           ( $ ( $ tt ) + ) ) ; } ) ; d ! ( end cbor root ) ;
                           (  ) } } ; ( @ $ enc : ident $ other : expr ) => {
                           $ other } ;);
        ///  TODO: Parse the vector e.g. array items
        #[macro_export]
        macro_rules! parse_vector(( $ ( $ content : tt ) * ) => {
                                  $ crate :: vec ! [ $ ( $ content ) * ] } ;);
        ///  Show an unexpected token error
        #[macro_export]
        macro_rules! unexpected_token((  ) => {  } ;);
        ///  Compose the payload root.
        #[macro_export]
        macro_rules! coap_root(( @ cbor $ context : ident $ children0 : block
                               ) => {
                               {
                               d ! ( begin cbor coap_root ) ; $ crate ::
                               oc_rep_start_root_object ! ( $ context ) ; $
                               children0 ; $ crate :: oc_rep_end_root_object !
                               ( $ context ) ; d ! ( end cbor coap_root ) ; }
                               } ; (
                               @ json $ context : ident $ children0 : block )
                               => {
                               {
                               d ! ( begin json coap_root ) ; unsafe {
                               sensor_coap :: json_rep_start_root_object (  )
                               } $ children0 ; unsafe {
                               sensor_coap :: json_rep_end_root_object (  ) }
                               d ! ( end json coap_root ) ; } } ;);
        ///  Compose an array under `object`, named as `key` (e.g. `values`).  Add `children` as array elements.
        #[macro_export]
        macro_rules! coap_array((
                                @ cbor $ object0 : ident , $ key0 : ident , $
                                children0 : block ) => {
                                {
                                d ! (
                                begin cbor coap_array , object : $ object0 ,
                                key : $ key0 ) ; $ crate :: oc_rep_set_array !
                                ( $ object0 , $ key0 ) ; $ children0 ; $ crate
                                :: oc_rep_close_array ! ( $ object0 , $ key0 )
                                ; d ! ( end cbor coap_array ) ; } } ; (
                                @ json $ object0 : ident , $ key0 : ident , $
                                children0 : block ) => {
                                {
                                d ! (
                                begin json coap_array , object : $ object0 ,
                                key : $ key0 ) ; $ crate :: json_rep_set_array
                                ! ( $ object0 , $ key0 ) ; $ children0 ; $
                                crate :: json_rep_close_array ! (
                                $ object0 , $ key0 ) ; d ! (
                                end json coap_array ) ; } } ;);
        ///  Append a (key + int value) item to the array named `array`:
        ///    `{ <array>: [ ..., {"key": <key0>, "value": <value0>} ], ... }`
        #[macro_export]
        macro_rules! coap_item_int((
                                   @ cbor $ array0 : ident , $ key0 : expr , $
                                   value0 : expr ) => {
                                   {
                                   d ! (
                                   begin cbor coap_item_int , key : $ key0 ,
                                   value : $ value0 ) ; $ crate :: coap_item !
                                   (
                                   @ cbor $ array0 , {
                                   $ crate :: oc_rep_set_text_string ! (
                                   $ array0 , "key" , $ key0 ) ; $ crate ::
                                   oc_rep_set_int ! (
                                   $ array0 , "value" , $ value0 ) ; } ) ; d !
                                   ( end cbor coap_item_int ) ; } } ; (
                                   @ json $ array0 : ident , $ key0 : expr , $
                                   value0 : expr ) => {
                                   {
                                   d ! (
                                   begin json coap_item_int , key : $ key0 ,
                                   value : $ value0 ) ; $ crate :: coap_item !
                                   (
                                   @ json $ array0 , {
                                   $ crate :: json_rep_set_text_string ! (
                                   $ array0 , "key" , $ key0 ) ; $ crate ::
                                   json_rep_set_int ! (
                                   $ array0 , "value" , $ value0 ) ; } ) ; d !
                                   ( end json coap_item_int ) ; } } ;);
        ///  Append a (`key` + `val` string value) item to the array named `parent`:
        ///    `{ <parent>: [ ..., {"key": <key>, "value": <val>} ] }`
        #[macro_export]
        macro_rules! coap_item_str((
                                   @ cbor $ parent : ident , $ key : expr , $
                                   val : expr ) => {
                                   {
                                   d ! (
                                   begin cbor coap_item_str , parent : $
                                   parent , key : $ key , val : $ val ) ; $
                                   crate :: coap_item ! (
                                   @ cbor $ parent , {
                                   $ crate :: oc_rep_set_text_string ! (
                                   $ parent , "key" , $ key ) ; $ crate ::
                                   oc_rep_set_text_string ! (
                                   $ parent , "value" , $ val ) ; } ) ; d ! (
                                   end cbor coap_item_str ) ; } } ; (
                                   @ json $ parent : ident , $ key : expr , $
                                   val : expr ) => {
                                   {
                                   d ! (
                                   begin json coap_item_str , parent : $
                                   parent , key : $ key , val : $ val ) ; $
                                   crate :: coap_item ! (
                                   @ json $ parent , {
                                   $ crate :: json_rep_set_text_string ! (
                                   $ parent , key , $ key ) ; $ crate ::
                                   json_rep_set_text_string ! (
                                   $ parent , value , $ val ) ; } ) ; d ! (
                                   end json coap_item_str ) ; } } ;);
        ///  Append an array item under the current object item.  Add `children0` as the array items.
        ///    `{ <array0>: [ ..., { <children0> } ] }`
        #[macro_export]
        macro_rules! coap_item((
                               @ cbor $ context : ident , $ children0 : block
                               ) => {
                               {
                               d ! ( begin cbor coap_item , array : $ context
                               ) ; $ crate :: oc_rep_object_array_start_item !
                               ( $ context ) ; $ children0 ; $ crate ::
                               oc_rep_object_array_end_item ! ( $ context ) ;
                               d ! ( end cbor coap_item ) ; } } ; (
                               @ json $ context : ident , $ children0 : block
                               ) => {
                               {
                               d ! ( begin json coap_item , array : $ context
                               ) ; $ crate :: json_rep_object_array_start_item
                               ! ( $ context ) ; $ children0 ; $ crate ::
                               json_rep_object_array_end_item ! ( $ context )
                               ; d ! ( end json coap_item ) ; } } ;);
        ///  Given an object parent and an integer Sensor Value `val`, set the `val`'s key/value in the object.
        #[macro_export]
        macro_rules! coap_set_int_val((
                                      @ cbor $ context : ident , $ val0 : expr
                                      ) => {
                                      {
                                      d ! (
                                      begin cbor coap_set_int_val , c : $
                                      context , val : $ val0 ) ; if let
                                      SensorValueType :: Uint ( val ) = $ val0
                                      . val {
                                      $ crate :: oc_rep_set_int ! (
                                      $ context , $ val0 . key , val ) ; }
                                      else {
                                      unsafe {
                                      $ context . fail (
                                      json_context :: JsonError ::
                                      VALUE_NOT_UINT ) } ; } d ! (
                                      end cbor coap_set_int_val ) ; } } ; (
                                      @ json $ context : ident , $ val0 : expr
                                      ) => {
                                      {
                                      d ! (
                                      begin json coap_set_int_val , c : $
                                      context , val : $ val0 ) ; if let
                                      SensorValueType :: Uint ( val ) = $ val0
                                      . val {
                                      $ crate :: json_rep_set_int ! (
                                      $ context , $ val0 . key , val ) ; }
                                      else {
                                      unsafe {
                                      $ context . fail (
                                      json_context :: JsonError ::
                                      VALUE_NOT_UINT ) } ; } d ! (
                                      end json coap_set_int_val ) ; } } ;);
        ///  Create a new Item object in the parent array and set the Sensor Value's key/value (integer).
        #[macro_export]
        macro_rules! coap_item_int_val((
                                       @ cbor $ context : ident , $ val0 :
                                       expr ) => {
                                       {
                                       d ! (
                                       begin cbor coap_item_int_val , c : $
                                       context , val : $ val0 ) ; if let
                                       SensorValueType :: Uint ( val ) = $
                                       val0 . val {
                                       $ crate :: coap_item_int ! (
                                       @ cbor $ context , $ val0 . key , val )
                                       ; } else {
                                       unsafe {
                                       $ context . fail (
                                       json_context :: JsonError ::
                                       VALUE_NOT_UINT ) } ; } d ! (
                                       end cbor coap_item_int_val ) ; } } ; (
                                       @ json $ context : ident , $ val0 :
                                       expr ) => {
                                       {
                                       d ! (
                                       begin json coap_item_int_val , c : $
                                       context , val : $ val0 ) ; if let
                                       SensorValueType :: Uint ( val ) = $
                                       val0 . val {
                                       $ crate :: coap_item_int ! (
                                       @ json $ context , $ val0 . key , val )
                                       ; } else {
                                       unsafe {
                                       $ context . fail (
                                       json_context :: JsonError ::
                                       VALUE_NOT_UINT ) } ; } d ! (
                                       end json coap_item_int_val ) ; } } ;);
        ///  Assume we are writing an object now.  Write the key name and start a child array.
        ///  ```
        ///  {a:b --> {a:b, key:[
        ///  ```
        #[macro_export]
        macro_rules! json_rep_set_array(( $ context : ident , $ key : ident )
                                        => {
                                        {
                                        concat ! (
                                        "<< jarri " , ", o: " , stringify ! (
                                        $ context ) , ", k: " , stringify ! (
                                        $ key ) ) ; let key_with_null : & str
                                        = $ crate :: stringify_null ! ( $ key
                                        ) ; unsafe {
                                        mynewt_rust :: json_helper_set_array (
                                        $ context . to_void_ptr (  ) , $
                                        context . key_to_cstr (
                                        key_with_null . as_bytes (  ) ) ) ; }
                                        ; } } ; (
                                        $ context : ident , $ key : expr ) =>
                                        {
                                        {
                                        concat ! (
                                        "<< jarre " , ", o: " , stringify ! (
                                        $ context ) , ", k: " , stringify ! (
                                        $ key ) ) ; let key_with_opt_null : &
                                        [ u8 ] = $ key . to_bytes_optional_nul
                                        (  ) ; unsafe {
                                        mynewt_rust :: json_helper_set_array (
                                        $ context . to_void_ptr (  ) , $
                                        context . key_to_cstr (
                                        key_with_opt_null ) ) ; } ; } } ;);
        ///  End the child array and resume writing the parent object.
        ///  ```
        ///  {a:b, key:[... --> {a:b, key:[...]
        ///  ```
        #[macro_export]
        macro_rules! json_rep_close_array(( $ context : ident , $ key : ident
                                          ) => {
                                          {
                                          concat ! ( ">>" ) ; let
                                          key_with_null : & str = $ crate ::
                                          stringify_null ! ( $ key ) ; unsafe
                                          {
                                          mynewt_rust ::
                                          json_helper_close_array (
                                          $ context . to_void_ptr (  ) , $
                                          context . key_to_cstr (
                                          key_with_null . as_bytes (  ) ) ) }
                                          ; } } ; (
                                          $ context : ident , $ key : expr )
                                          => {
                                          {
                                          concat ! ( ">>" ) ; let
                                          key_with_opt_null : & [ u8 ] = $ key
                                          . to_bytes_optional_nul (  ) ;
                                          unsafe {
                                          mynewt_rust ::
                                          json_helper_close_array (
                                          $ context . to_void_ptr (  ) , $
                                          context . key_to_cstr (
                                          key_with_opt_null ) ) } ; } } ;);
        ///  Assume we have called `set_array`.  Start an array item, assumed to be an object.
        ///  ```
        ///  [... --> [...,
        ///  ```
        #[macro_export]
        macro_rules! json_rep_object_array_start_item(( $ context : ident ) =>
                                                      {
                                                      {
                                                      concat ! (
                                                      "<< jitmi" , " c: " ,
                                                      stringify ! ( $ context
                                                      ) ) ; let key_with_null
                                                      : & str = $ crate ::
                                                      stringify_null ! (
                                                      $ context ) ; unsafe {
                                                      mynewt_rust ::
                                                      json_helper_object_array_start_item
                                                      (
                                                      $ context . key_to_cstr
                                                      (
                                                      key_with_null . as_bytes
                                                      (  ) ) ) } ; } } ; (
                                                      $ context : ident ) => {
                                                      {
                                                      concat ! (
                                                      "<< jitme" , " c: " ,
                                                      stringify ! ( $ context
                                                      ) ) ; let
                                                      key_with_opt_null : & [
                                                      u8 ] = $ context .
                                                      to_bytes_optional_nul (
                                                      ) ; unsafe {
                                                      mynewt_rust ::
                                                      json_helper_object_array_start_item
                                                      (
                                                      $ context . key_to_cstr
                                                      ( key_with_opt_null ) )
                                                      } ; } } ;);
        ///  End an array item, assumed to be an object.
        ///  ```
        ///  [... --> [...,
        ///  ```
        #[macro_export]
        macro_rules! json_rep_object_array_end_item(( $ context : ident ) => {
                                                    {
                                                    concat ! ( ">>" ) ; let
                                                    key_with_null : & str = $
                                                    crate :: stringify_null !
                                                    ( $ context ) ; unsafe {
                                                    mynewt_rust ::
                                                    json_helper_object_array_end_item
                                                    (
                                                    $ context . key_to_cstr (
                                                    key_with_null . as_bytes (
                                                     ) ) ) } ; } } ; (
                                                    $ context : ident ) => {
                                                    {
                                                    concat ! ( ">>" ) ; let
                                                    key_with_opt_null : & [ u8
                                                    ] = $ context .
                                                    to_bytes_optional_nul (  )
                                                    ; unsafe {
                                                    mynewt_rust ::
                                                    json_helper_object_array_end_item
                                                    (
                                                    $ context . key_to_cstr (
                                                    key_with_opt_null ) ) } ;
                                                    } } ;);
        ///  Encode an int value into the current JSON encoding value `coap_json_value`
        #[macro_export]
        macro_rules! json_rep_set_int((
                                      $ context : ident , $ key : ident , $
                                      value : expr ) => {
                                      {
                                      concat ! (
                                      "-- jinti" , " o: " , stringify ! (
                                      $ context ) , ", k: " , stringify ! (
                                      $ key ) , ", v: " , stringify ! (
                                      $ value ) ) ; let key_with_null : & str
                                      = $ crate :: stringify_null ! ( $ key )
                                      ; let value = $ value as u64 ; unsafe {
                                      mynewt_rust :: json_helper_set_int (
                                      $ context . to_void_ptr (  ) , $ context
                                      . key_to_cstr (
                                      key_with_null . as_bytes (  ) ) , value
                                      ) } ; } } ; (
                                      $ context : ident , $ key : expr , $
                                      value : expr ) => {
                                      {
                                      concat ! (
                                      "-- jinte" , " o: " , stringify ! (
                                      $ context ) , ", k: " , stringify ! (
                                      $ key ) , ", v: " , stringify ! (
                                      $ value ) ) ; let key_with_opt_null : &
                                      [ u8 ] = $ key . to_bytes_optional_nul (
                                       ) ; let value = $ value as u64 ; unsafe
                                      {
                                      mynewt_rust :: json_helper_set_int (
                                      $ context . to_void_ptr (  ) , $ context
                                      . key_to_cstr ( key_with_opt_null ) ,
                                      value ) } ; } } ;);
        ///  Encode a text value into the current JSON encoding value `coap_json_value`
        #[macro_export]
        macro_rules! json_rep_set_text_string((
                                              $ context : ident , $ key :
                                              ident , $ value : expr ) => {
                                              {
                                              concat ! (
                                              "-- jtxti" , " o: " , stringify
                                              ! ( $ context ) , ", k: " ,
                                              stringify ! ( $ key ) , ", v: "
                                              , stringify ! ( $ value ) ) ;
                                              let key_with_null : & str = $
                                              crate :: stringify_null ! (
                                              $ key ) ; let
                                              value_with_opt_null : & [ u8 ] =
                                              $ value . to_bytes_optional_nul
                                              (  ) ; unsafe {
                                              mynewt_rust ::
                                              json_helper_set_text_string (
                                              $ context . to_void_ptr (  ) , $
                                              context . key_to_cstr (
                                              key_with_null . as_bytes (  ) )
                                              , $ context . value_to_cstr (
                                              value_with_opt_null ) ) } ; } }
                                              ; (
                                              $ context : ident , $ key : expr
                                              , $ value : expr ) => {
                                              {
                                              concat ! (
                                              "-- jtxte" , " o: " , stringify
                                              ! ( $ context ) , ", k: " ,
                                              stringify ! ( $ key ) , ", v: "
                                              , stringify ! ( $ value ) ) ;
                                              let key_with_opt_null : & [ u8 ]
                                              = $ key . to_bytes_optional_nul
                                              (  ) ; let value_with_opt_null :
                                              & [ u8 ] = $ value .
                                              to_bytes_optional_nul (  ) ;
                                              unsafe {
                                              mynewt_rust ::
                                              json_helper_set_text_string (
                                              $ context . to_void_ptr (  ) , $
                                              context . key_to_cstr (
                                              key_with_opt_null ) , $ context
                                              . value_to_cstr (
                                              value_with_opt_null ) ) } ; } }
                                              ;);
        #[macro_export]
        macro_rules! oc_rep_start_root_object(( $ context : ident ) => {
                                              {
                                              d ! (
                                              begin oc_rep_start_root_object )
                                              ; unsafe {
                                              let encoder = $ context .
                                              encoder ( "root" , "_map" ) ;
                                              tinycbor ::
                                              cbor_encoder_create_map (
                                              $ context . global_encoder (  )
                                              , encoder , tinycbor ::
                                              CborIndefiniteLength ) } ; d ! (
                                              end oc_rep_start_root_object ) ;
                                              } } ;);
        #[macro_export]
        macro_rules! oc_rep_end_root_object(( $ context : ident ) => {
                                            {
                                            d ! ( begin oc_rep_end_root_object
                                            ) ; unsafe {
                                            let encoder = $ context . encoder
                                            ( "root" , "_map" ) ; tinycbor ::
                                            cbor_encoder_close_container (
                                            $ context . global_encoder (  ) ,
                                            encoder ) } ; d ! (
                                            end oc_rep_end_root_object ) ; } }
                                            ;);
        #[macro_export]
        macro_rules! oc_rep_start_object((
                                         $ parent : ident , $ key : ident , $
                                         parent_suffix : ident ) => {
                                         {
                                         concat ! (
                                         "begin oc_rep_start_object " ,
                                         ", parent: " , stringify ! ( $ parent
                                         ) , stringify ! ( $ parent_suffix ) ,
                                         ", key: " , stringify ! ( $ key ) ,
                                         ", child: " , stringify ! ( $ key ) ,
                                         "_map" ) ; unsafe {
                                         let encoder = $ context . encoder (
                                         stringify ! ( $ key ) , "_map" ) ;
                                         tinycbor :: cbor_encoder_create_map (
                                         encoder , & mut concat_idents ! (
                                         $ key , _map ) , tinycbor ::
                                         CborIndefiniteLength ) ; } ; d ! (
                                         end oc_rep_start_object ) ; } } ;);
        #[macro_export]
        macro_rules! oc_rep_end_object((
                                       $ parent : ident , $ key : ident , $
                                       parent_suffix : ident ) => {
                                       {
                                       concat ! (
                                       "begin oc_rep_end_object " ,
                                       ", parent: " , stringify ! ( $ parent )
                                       , stringify ! ( $ parent_suffix ) ,
                                       ", key: " , stringify ! ( $ key ) ,
                                       ", child: " , stringify ! ( $ key ) ,
                                       "_map" ) ; unsafe {
                                       let encoder = $ context . encoder (
                                       stringify ! ( $ key ) , "_map" ) ;
                                       tinycbor ::
                                       cbor_encoder_close_container (
                                       encoder , & mut concat_idents ! (
                                       $ key , _map ) ) ; } ; d ! (
                                       end oc_rep_end_object ) ; } } ;);
        #[macro_export]
        macro_rules! oc_rep_start_array((
                                        $ parent : ident , $ key : ident , $
                                        parent_suffix : ident ) => {
                                        {
                                        concat ! (
                                        "begin oc_rep_start_array " ,
                                        ", parent: " , stringify ! ( $ parent
                                        ) , stringify ! ( $ parent_suffix ) ,
                                        ", key: " , stringify ! ( $ key ) ,
                                        ", child: " , stringify ! ( $ key ) ,
                                        "_array" ) ; unsafe {
                                        tinycbor :: cbor_encoder_create_array
                                        (
                                        & mut concat_idents ! (
                                        $ parent , $ parent_suffix ) , & mut
                                        concat_idents ! ( $ key , _array ) ,
                                        CborIndefiniteLength ) } ; d ! (
                                        end oc_rep_start_array ) ; } } ;);
        #[macro_export]
        macro_rules! oc_rep_end_array((
                                      $ parent : ident , $ key : ident , $
                                      parent_suffix : ident ) => {
                                      {
                                      concat ! (
                                      "begin oc_rep_end_array " , ", parent: "
                                      , stringify ! ( $ parent ) , stringify !
                                      ( $ parent_suffix ) , ", key: " ,
                                      stringify ! ( $ key ) , ", child: " ,
                                      stringify ! ( $ key ) , "_array" ) ;
                                      unsafe {
                                      tinycbor :: cbor_encoder_close_container
                                      (
                                      & mut $ parent , & mut concat_idents ! (
                                      $ parent , $ parent_suffix ) ) } ; d ! (
                                      end oc_rep_end_array ) ; } } ;);
        ///  Assume we are writing an object now.  Write the key name and start a child array.
        ///  ```
        ///  {a:b --> {a:b, key:[
        ///  ```
        #[macro_export]
        macro_rules! oc_rep_set_array(( $ object : ident , $ key : ident ) =>
                                      {
                                      {
                                      concat ! (
                                      "begin oc_rep_set_array " , ", object: "
                                      , stringify ! ( $ object ) , ", key: " ,
                                      stringify ! ( $ key ) , ", child: " ,
                                      stringify ! ( $ object ) , "_map" ) ;
                                      unsafe {
                                      cbor_encode_text_string (
                                      & mut concat_idents ! ( $ object , _map
                                      ) , $ key . as_ptr (  ) , $ key . len (
                                      ) ) } ; oc_rep_start_array ! (
                                      $ object , $ key , _map ) ; d ! (
                                      end oc_rep_set_array ) ; } } ;);
        ///  End the child array and resume writing the parent object.
        ///  ```
        ///  {a:b, key:[... --> {a:b, key:[...]
        ///  ```
        #[macro_export]
        macro_rules! oc_rep_close_array(( $ object : ident , $ key : ident )
                                        => {
                                        {
                                        concat ! (
                                        "begin oc_rep_close_array " ,
                                        ", object: " , stringify ! ( $ object
                                        ) , ", key: " , stringify ! ( $ key )
                                        , ", child: " , stringify ! ( $ object
                                        ) , "_map" ) ; oc_rep_end_array ! (
                                        $ object , $ key , _map ) ; d ! (
                                        end oc_rep_close_array ) ; } } ;);
        ///  Assume we have called `set_array`.  Start an array item, assumed to be an object.
        ///  ```
        ///  [... --> [...,
        ///  ```
        #[macro_export]
        macro_rules! oc_rep_object_array_start_item(( $ key : ident ) => {
                                                    {
                                                    concat ! (
                                                    "begin oc_rep_object_array_start_item "
                                                    , ", key: " , stringify !
                                                    ( $ key ) , ", child: " ,
                                                    stringify ! ( $ key ) ,
                                                    "_array" , ) ;
                                                    oc_rep_start_object ! (
                                                    $ key , $ key , _array ) ;
                                                    d ! (
                                                    end
                                                    oc_rep_object_array_start_item
                                                    ) ; } } ;);
        ///  End an array item, assumed to be an object.
        ///  ```
        ///  [... --> [...,
        ///  ```
        #[macro_export]
        macro_rules! oc_rep_object_array_end_item(( $ key : ident ) => {
                                                  {
                                                  concat ! (
                                                  "begin oc_rep_object_array_end_item "
                                                  , ", key: " , stringify ! (
                                                  $ key ) , ", child: " ,
                                                  stringify ! ( $ key ) ,
                                                  "_array" , ) ;
                                                  oc_rep_end_object ! (
                                                  $ key , $ key , _array ) ; d
                                                  ! (
                                                  end
                                                  oc_rep_object_array_end_item
                                                  ) ; } } ;);
        #[macro_export]
        macro_rules! run_stmts(( $ context : ident , $ encoder : ident , {  }
                               ) => { let _zzz = "aaa" ; } ; (
                               $ context : ident , $ encoder : ident , {
                               $ stmt : tt ; $ ( $ tail : tt ; ) * } ) => {
                               $ stmt ; $ crate :: run_stmts ! (
                               $ context , $ encoder , { $ ( $ tail ; ) * } )
                               } ;);
        #[macro_export]
        macro_rules! OLDrun((
                            $ context : ident , $ parent : ident , $ suffix :
                            expr , { $ ( $ stmt : stmt ; ) * } ) => {
                            concat ! (
                            " >> " , stringify ! ( $ context ) , " >> " ,
                            stringify ! ( $ parent ) , " >> " , stringify ! (
                            $ suffix ) ) ; unsafe {
                            $ crate :: run_stmts ! (
                            $ context , encoder , { $ ( $ stmt ; ) * } ) ; } ;
                            } ;);
        ///  Encode an int value 
        #[macro_export]
        macro_rules! oc_rep_set_int((
                                    $ context : ident , $ key : ident , $
                                    value : expr ) => {
                                    concat ! (
                                    "-- cinti" , " c: " , stringify ! (
                                    $ context ) , ", k: " , stringify ! (
                                    $ key ) , ", v: " , stringify ! ( $ value
                                    ) ) ; let key_with_null : & str = $ crate
                                    :: stringify_null ! ( $ key ) ; let value
                                    = $ value as i64 ; unsafe {
                                    let encoder = $ context . encoder (
                                    stringify ! ( $ context ) , "_map" ) ;
                                    tinycbor :: cbor_encode_text_string (
                                    encoder , $ context . key_to_cstr (
                                    key_with_null . as_bytes (  ) ) , $
                                    context . cstr_len (
                                    key_with_null . as_bytes (  ) ) ) ;
                                    tinycbor :: cbor_encode_int (
                                    encoder , value ) ; } } ; (
                                    $ context : ident , $ key : expr , $ value
                                    : expr ) => {
                                    concat ! (
                                    "-- cinte" , " c: " , stringify ! (
                                    $ context ) , ", k: " , stringify ! (
                                    $ key ) , ", v: " , stringify ! ( $ value
                                    ) ) ; let key_with_opt_null : & [ u8 ] = $
                                    key . to_bytes_optional_nul (  ) ; let
                                    value = $ value as i64 ;
                                    "-------------------------------------------------------------"
                                    ; mynewt_macros :: run ! (
                                    {
                                    let encoder = JSON_CONTEXT . encoder (
                                    "JSON_CONTEXT" , "_map" ) ;
                                    cbor_encode_text_string (
                                    encoder , JSON_CONTEXT . key_to_cstr (
                                    key_with_opt_null ) , JSON_CONTEXT .
                                    cstr_len ( key_with_opt_null ) ) ;
                                    cbor_encode_int ( encoder , value ) ; } )
                                    ;
                                    "-------------------------------------------------------------"
                                    ; } ;);
        ///  Encode a text value 
        #[macro_export]
        macro_rules! oc_rep_set_text_string((
                                            $ object : ident , $ key : expr ,
                                            $ value : expr ) => {
                                            {
                                            concat ! (
                                            "begin oc_rep_set_text_string " ,
                                            ", object: " , stringify ! (
                                            $ object ) , ", key: " , stringify
                                            ! ( $ key ) , ", value: " ,
                                            stringify ! ( $ value ) ,
                                            ", child: " , stringify ! (
                                            $ object ) , "_map" ) ; unsafe {
                                            cbor_encode_text_string (
                                            & mut concat_idents ! (
                                            $ object , _map ) , $ key . as_ptr
                                            (  ) , $ key . len (  ) ) ;
                                            cbor_encode_text_string (
                                            & mut concat_idents ! (
                                            $ object , _map ) , $ value .
                                            as_ptr (  ) , $ value . len (  ) )
                                            ; } d ! (
                                            end oc_rep_set_text_string ) ; } }
                                            ;);
        ///  Macro that takes an identifier and returns a `[u8]` containing the identifier, terminated by 0.
        ///  Used to convert an identifier to a C null-terminated string.
        #[macro_export]
        macro_rules! stringify_null(( $ key : ident ) => {
                                    concat ! ( stringify ! ( $ key ) , "\0" )
                                    } ;);
        ///  Macro to dump all tokens received as a literal string, e.g.
        ///  `d!(a b c)` returns `"a b c"`
        #[macro_export]
        macro_rules! d(( $ ( $ token : tt ) * ) => {
                       stringify ! ( $ ( $ token ) * ) } ;);
        ///  Macro to display the token being parsed and the remaining tokens
        #[macro_export]
        macro_rules! nx((
                        ( $ ( $ current : tt ) * ) , ( $ ( $ next : tt ) * ) ,
                        ( $ ( $ rest : tt ) * ) ) => {
                        concat ! (
                        " >> " , stringify ! ( $ ( $ current ) * ) , " >> " ,
                        stringify ! ( $ ( $ next ) * ) , " >> " , stringify !
                        ( $ ( $ rest ) * ) ) ; } ;);
    }
    pub mod encoding {
        //! Mynewt Encoding API for Rust
        /// Contains Rust bindings for Mynewt JSON Encoding API `encoding/json`
        pub mod json {
            #[repr(C)]
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub struct __BindgenBitfieldUnit<Storage, Align> where
                       Storage: AsRef<[u8]> + AsMut<[u8]> {
                storage: Storage,
                align: [Align; 0],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::marker::Copy, Align: ::core::marker::Copy>
             ::core::marker::Copy for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::clone::Clone, Align: ::core::clone::Clone>
             ::core::clone::Clone for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn clone(&self) -> __BindgenBitfieldUnit<Storage, Align> {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } =>
                        __BindgenBitfieldUnit{storage:
                                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                                              align:
                                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::fmt::Debug, Align: ::core::fmt::Debug>
             ::core::fmt::Debug for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter)
                 -> ::core::fmt::Result {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            let mut debug_trait_builder =
                                f.debug_struct("__BindgenBitfieldUnit");
                            let _ =
                                debug_trait_builder.field("storage",
                                                          &&(*__self_0_0));
                            let _ =
                                debug_trait_builder.field("align",
                                                          &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::default::Default,
                  Align: ::core::default::Default> ::core::default::Default
             for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn default() -> __BindgenBitfieldUnit<Storage, Align> {
                    __BindgenBitfieldUnit{storage:
                                              ::core::default::Default::default(),
                                          align:
                                              ::core::default::Default::default(),}
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Eq, Align: ::core::cmp::Eq>
             ::core::cmp::Eq for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Storage>;
                        let _: ::core::cmp::AssertParamIsEq<[Align; 0]>;
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::hash::Hash, Align: ::core::hash::Hash>
             ::core::hash::Hash for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn hash<__HStorageAlign: ::core::hash::Hasher>(&self,
                                                               state:
                                                                   &mut __HStorageAlign)
                 -> () {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            ::core::hash::Hash::hash(&(*__self_0_0), state);
                            ::core::hash::Hash::hash(&(*__self_0_1), state)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Ord, Align: ::core::cmp::Ord>
             ::core::cmp::Ord for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn cmp(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::cmp::Ordering {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                        &(*__self_1_0)) {
                                ::core::cmp::Ordering::Equal =>
                                match ::core::cmp::Ord::cmp(&(*__self_0_1),
                                                            &(*__self_1_1)) {
                                    ::core::cmp::Ordering::Equal =>
                                    ::core::cmp::Ordering::Equal,
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialEq,
                  Align: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn eq(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialOrd,
                  Align: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn partial_cmp(&self,
                               other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::option::Option<::core::cmp::Ordering> {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                       &(*__self_1_0))
                                {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                =>
                                match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                           &(*__self_1_1))
                                    {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                    =>
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
                #[inline]
                fn lt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                == ::core::cmp::Ordering::Less,
                        },
                    }
                }
                #[inline]
                fn le(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                != ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn gt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                == ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn ge(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                != ::core::cmp::Ordering::Less,
                        },
                    }
                }
            }
            impl <Storage, Align> __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                pub fn new(storage: Storage) -> Self {
                    Self{storage, align: [],}
                }
                #[inline]
                pub fn get_bit(&self, index: usize) -> bool {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           22u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = self.storage.as_ref()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    byte & mask == mask
                }
                #[inline]
                pub fn set_bit(&mut self, index: usize, val: bool) {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           35u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = &mut self.storage.as_mut()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    if val { *byte |= mask; } else { *byte &= !mask; }
                }
                #[inline]
                pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/encoding/json.rs",
                                                           52u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           53u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           54u32, 9u32))
                            }
                        };
                    };
                    let mut val = 0;
                    for i in 0..(bit_width as usize) {
                        if self.get_bit(i + bit_offset) {
                            let index =
                                if false {
                                    bit_width as usize - 1 - i
                                } else { i };
                            val |= 1 << index;
                        }
                    }
                    val
                }
                #[inline]
                pub fn set(&mut self, bit_offset: usize, bit_width: u8,
                           val: u64) {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/encoding/json.rs",
                                                           70u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           71u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/encoding/json.rs",
                                                           72u32, 9u32))
                            }
                        };
                    };
                    for i in 0..(bit_width as usize) {
                        let mask = 1 << i;
                        let val_bit_is_set = val & mask == mask;
                        let index =
                            if false {
                                bit_width as usize - 1 - i
                            } else { i };
                        self.set_bit(index + bit_offset, val_bit_is_set);
                    }
                }
            }
            #[repr(C)]
            pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
            impl <T> __BindgenUnionField<T> {
                #[inline]
                pub fn new() -> Self {
                    __BindgenUnionField(::core::marker::PhantomData)
                }
                #[inline]
                pub unsafe fn as_ref(&self) -> &T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_mut(&mut self) -> &mut T {
                    ::core::mem::transmute(self)
                }
            }
            impl <T> ::core::default::Default for __BindgenUnionField<T> {
                #[inline]
                fn default() -> Self { Self::new() }
            }
            impl <T> ::core::clone::Clone for __BindgenUnionField<T> {
                #[inline]
                fn clone(&self) -> Self { Self::new() }
            }
            impl <T> ::core::marker::Copy for __BindgenUnionField<T> { }
            impl <T> ::core::fmt::Debug for __BindgenUnionField<T> {
                fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
                 -> ::core::fmt::Result {
                    fmt.write_str("__BindgenUnionField")
                }
            }
            impl <T> ::core::hash::Hash for __BindgenUnionField<T> {
                fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) { }
            }
            impl <T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
                fn eq(&self, _other: &__BindgenUnionField<T>) -> bool { true }
            }
            impl <T> ::core::cmp::Eq for __BindgenUnionField<T> { }
            pub const JSON_VALUE_TYPE_BOOL: u32 = 0;
            pub const JSON_VALUE_TYPE_UINT64: u32 = 1;
            pub const JSON_VALUE_TYPE_INT64: u32 = 2;
            pub const JSON_VALUE_TYPE_STRING: u32 = 3;
            pub const JSON_VALUE_TYPE_ARRAY: u32 = 4;
            pub const JSON_VALUE_TYPE_OBJECT: u32 = 5;
            pub const JSON_ATTR_MAX: u32 = 31;
            pub const JSON_VAL_MAX: u32 = 512;
            pub const JSON_ERR_OBSTART: u32 = 1;
            pub const JSON_ERR_ATTRSTART: u32 = 2;
            pub const JSON_ERR_BADATTR: u32 = 3;
            pub const JSON_ERR_ATTRLEN: u32 = 4;
            pub const JSON_ERR_NOARRAY: u32 = 5;
            pub const JSON_ERR_NOBRAK: u32 = 6;
            pub const JSON_ERR_STRLONG: u32 = 7;
            pub const JSON_ERR_TOKLONG: u32 = 8;
            pub const JSON_ERR_BADTRAIL: u32 = 9;
            pub const JSON_ERR_ARRAYSTART: u32 = 10;
            pub const JSON_ERR_OBJARR: u32 = 11;
            pub const JSON_ERR_SUBTOOLONG: u32 = 12;
            pub const JSON_ERR_BADSUBTRAIL: u32 = 13;
            pub const JSON_ERR_SUBTYPE: u32 = 14;
            pub const JSON_ERR_BADSTRING: u32 = 15;
            pub const JSON_ERR_CHECKFAIL: u32 = 16;
            pub const JSON_ERR_NOPARSTR: u32 = 17;
            pub const JSON_ERR_BADENUM: u32 = 18;
            pub const JSON_ERR_QNONSTRING: u32 = 19;
            pub const JSON_ERR_NONQSTRING: u32 = 19;
            pub const JSON_ERR_MISC: u32 = 20;
            pub const JSON_ERR_BADNUM: u32 = 21;
            pub const JSON_ERR_NULLPTR: u32 = 22;
            pub type __uint8_t = ::cty::c_uchar;
            pub type __uint16_t = ::cty::c_ushort;
            pub type __uint64_t = ::cty::c_ulonglong;
            #[repr(C)]
            pub struct json_value {
                pub jv_pad1: u8,
                pub jv_type: u8,
                pub jv_len: u16,
                pub jv_val: json_value__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct json_value__bindgen_ty_1 {
                pub u: __BindgenUnionField<u64>,
                pub fl: __BindgenUnionField<f32>,
                pub str: __BindgenUnionField<*mut ::cty::c_char>,
                pub composite: __BindgenUnionField<json_value__bindgen_ty_1__bindgen_ty_1>,
                pub bindgen_union_field: [u64; 2usize],
            }
            #[repr(C)]
            pub struct json_value__bindgen_ty_1__bindgen_ty_1 {
                pub keys: *mut *mut ::cty::c_char,
                pub values: *mut *mut json_value,
            }
            impl Default for json_value__bindgen_ty_1__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_value__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_value {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub type json_write_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(buf:
                                                                *mut ::cty::c_void,
                                                            data:
                                                                *mut ::cty::c_char,
                                                            len: ::cty::c_int)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct json_encoder {
                pub je_write: json_write_func_t,
                pub je_arg: *mut ::cty::c_void,
                pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                pub je_encode_buf: [::cty::c_char; 64usize],
            }
            impl Default for json_encoder {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl json_encoder {
                #[inline]
                pub fn je_wr_commas(&self) -> ::cty::c_int {
                    unsafe {
                        ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                    1u8) as
                                                   u32)
                    }
                }
                #[inline]
                pub fn set_je_wr_commas(&mut self, val: ::cty::c_int) {
                    unsafe {
                        let val: u32 = ::core::mem::transmute(val);
                        self._bitfield_1.set(0usize, 1u8, val as u64)
                    }
                }
                #[inline]
                pub fn new_bitfield_1(je_wr_commas: ::cty::c_int)
                 -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                    let mut __bindgen_bitfield_unit:
                            __BindgenBitfieldUnit<[u8; 1usize], u8> =
                        Default::default();
                    __bindgen_bitfield_unit.set(0usize, 1u8,
                                                {
                                                    let je_wr_commas: u32 =
                                                        unsafe {
                                                            ::core::mem::transmute(je_wr_commas)
                                                        };
                                                    je_wr_commas as u64
                                                });
                    __bindgen_bitfield_unit
                }
            }
            extern "C" {
                pub fn json_encode_object_start(arg1: *mut json_encoder)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_object_key(encoder: *mut json_encoder,
                                              key: *mut ::cty::c_char)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_object_entry(arg1: *mut json_encoder,
                                                arg2: *mut ::cty::c_char,
                                                arg3: *mut json_value)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_object_finish(arg1: *mut json_encoder)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_array_name(encoder: *mut json_encoder,
                                              name: *mut ::cty::c_char)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_array_start(encoder: *mut json_encoder)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_array_value(encoder: *mut json_encoder,
                                               val: *mut json_value)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_encode_array_finish(encoder: *mut json_encoder)
                 -> ::cty::c_int;
            }
            pub const json_type_t_integer: json_type = 0;
            pub const json_type_t_uinteger: json_type = 1;
            pub const json_type_t_real: json_type = 2;
            pub const json_type_t_string: json_type = 3;
            pub const json_type_t_boolean: json_type = 4;
            pub const json_type_t_character: json_type = 5;
            pub const json_type_t_object: json_type = 6;
            pub const json_type_t_structobject: json_type = 7;
            pub const json_type_t_array: json_type = 8;
            pub const json_type_t_check: json_type = 9;
            pub const json_type_t_ignore: json_type = 10;
            pub type json_type = u32;
            #[repr(C)]
            pub struct json_enum_t {
                pub name: *mut ::cty::c_char,
                pub value: ::cty::c_longlong,
            }
            impl Default for json_enum_t {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t {
                pub element_type: json_type,
                pub arr: json_array_t__bindgen_ty_1,
                pub count: *mut ::cty::c_int,
                pub maxlen: ::cty::c_int,
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1 {
                pub objects: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_1>,
                pub strings: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_2>,
                pub integers: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_3>,
                pub uintegers: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_4>,
                pub reals: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_5>,
                pub booleans: __BindgenUnionField<json_array_t__bindgen_ty_1__bindgen_ty_6>,
                pub bindgen_union_field: [u64; 3usize],
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_1 {
                pub subtype: *const json_attr_t,
                pub base: *mut ::cty::c_char,
                pub stride: usize,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_2 {
                pub ptrs: *mut *mut ::cty::c_char,
                pub store: *mut ::cty::c_char,
                pub storelen: ::cty::c_int,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_3 {
                pub store: *mut ::cty::c_longlong,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_3 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_4 {
                pub store: *mut ::cty::c_ulonglong,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_4 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_5 {
                pub store: *mut f64,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_5 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_array_t__bindgen_ty_1__bindgen_ty_6 {
                pub store: *mut bool,
            }
            impl Default for json_array_t__bindgen_ty_1__bindgen_ty_6 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_array_t__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_array_t {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_attr_t {
                pub attribute: *mut ::cty::c_char,
                pub type_: json_type,
                pub addr: json_attr_t__bindgen_ty_1,
                pub dflt: json_attr_t__bindgen_ty_2,
                pub len: usize,
                pub map: *const json_enum_t,
                pub nodefault: bool,
            }
            #[repr(C)]
            pub struct json_attr_t__bindgen_ty_1 {
                pub integer: __BindgenUnionField<*mut ::cty::c_longlong>,
                pub uinteger: __BindgenUnionField<*mut ::cty::c_ulonglong>,
                pub real: __BindgenUnionField<*mut f64>,
                pub string: __BindgenUnionField<*mut ::cty::c_char>,
                pub boolean: __BindgenUnionField<*mut bool>,
                pub character: __BindgenUnionField<*mut ::cty::c_char>,
                pub array: __BindgenUnionField<json_array_t>,
                pub offset: __BindgenUnionField<usize>,
                pub bindgen_union_field: [u64; 6usize],
            }
            impl Default for json_attr_t__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct json_attr_t__bindgen_ty_2 {
                pub integer: __BindgenUnionField<::cty::c_longlong>,
                pub uinteger: __BindgenUnionField<::cty::c_ulonglong>,
                pub real: __BindgenUnionField<f64>,
                pub boolean: __BindgenUnionField<bool>,
                pub character: __BindgenUnionField<::cty::c_char>,
                pub check: __BindgenUnionField<*mut ::cty::c_char>,
                pub bindgen_union_field: u64,
            }
            impl Default for json_attr_t__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_attr_t {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub type json_buffer_read_next_byte_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut json_buffer)
                                           -> ::cty::c_char>;
            pub type json_buffer_read_prev_byte_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut json_buffer)
                                           -> ::cty::c_char>;
            pub type json_buffer_readn_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut json_buffer,
                                                            buf:
                                                                *mut ::cty::c_char,
                                                            n: ::cty::c_int)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct json_buffer {
                pub jb_readn: json_buffer_readn_t,
                pub jb_read_next: json_buffer_read_next_byte_t,
                pub jb_read_prev: json_buffer_read_prev_byte_t,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for json_buffer {
                #[inline]
                fn default() -> json_buffer {
                    json_buffer{jb_readn: ::core::default::Default::default(),
                                jb_read_next:
                                    ::core::default::Default::default(),
                                jb_read_prev:
                                    ::core::default::Default::default(),}
                }
            }
            extern "C" {
                pub fn json_read_object(arg1: *mut json_buffer,
                                        arg2: *const json_attr_t)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn json_read_array(arg1: *mut json_buffer,
                                       arg2: *const json_array_t)
                 -> ::cty::c_int;
            }
        }
        /// Contains Rust bindings for Mynewt TinyCBOR Encoding API `encoding/tinycbor`
        pub mod tinycbor {
            pub const CborIndefiniteLength: usize = 0xffffffffusize;
            pub type __uint8_t = ::cty::c_uchar;
            pub type __uint16_t = ::cty::c_ushort;
            pub type __uint32_t = ::cty::c_ulong;
            pub type __int64_t = ::cty::c_longlong;
            pub type __uint64_t = ::cty::c_ulonglong;
            pub type __uintptr_t = ::cty::c_uint;
            pub type FILE = File;
            #[repr(C)]
            pub struct File_methods {
                pub write: ::core::option::Option<unsafe extern "C" fn(instance:
                                                                           *mut FILE,
                                                                       bp:
                                                                           *const ::cty::c_char,
                                                                       n:
                                                                           usize)
                                                      -> usize>,
                pub read: ::core::option::Option<unsafe extern "C" fn(instance:
                                                                          *mut FILE,
                                                                      bp:
                                                                          *mut ::cty::c_char,
                                                                      n:
                                                                          usize)
                                                     -> usize>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for File_methods {
                #[inline]
                fn default() -> File_methods {
                    File_methods{write: ::core::default::Default::default(),
                                 read: ::core::default::Default::default(),}
                }
            }
            #[repr(C)]
            pub struct File {
                pub vmt: *const File_methods,
            }
            impl Default for File {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub const CborType_CborIntegerType: CborType = 0;
            pub const CborType_CborByteStringType: CborType = 64;
            pub const CborType_CborTextStringType: CborType = 96;
            pub const CborType_CborArrayType: CborType = 128;
            pub const CborType_CborMapType: CborType = 160;
            pub const CborType_CborTagType: CborType = 192;
            pub const CborType_CborSimpleType: CborType = 224;
            pub const CborType_CborBooleanType: CborType = 245;
            pub const CborType_CborNullType: CborType = 246;
            pub const CborType_CborUndefinedType: CborType = 247;
            pub const CborType_CborHalfFloatType: CborType = 249;
            pub const CborType_CborFloatType: CborType = 250;
            pub const CborType_CborDoubleType: CborType = 251;
            pub const CborType_CborInvalidType: CborType = 255;
            pub type CborType = u32;
            pub type CborTag = u64;
            pub const CborKnownTags_CborDateTimeStringTag: CborKnownTags = 0;
            pub const CborKnownTags_CborUnixTime_tTag: CborKnownTags = 1;
            pub const CborKnownTags_CborPositiveBignumTag: CborKnownTags = 2;
            pub const CborKnownTags_CborNegativeBignumTag: CborKnownTags = 3;
            pub const CborKnownTags_CborDecimalTag: CborKnownTags = 4;
            pub const CborKnownTags_CborBigfloatTag: CborKnownTags = 5;
            pub const CborKnownTags_CborExpectedBase64urlTag: CborKnownTags =
                21;
            pub const CborKnownTags_CborExpectedBase64Tag: CborKnownTags = 22;
            pub const CborKnownTags_CborExpectedBase16Tag: CborKnownTags = 23;
            pub const CborKnownTags_CborUriTag: CborKnownTags = 32;
            pub const CborKnownTags_CborBase64urlTag: CborKnownTags = 33;
            pub const CborKnownTags_CborBase64Tag: CborKnownTags = 34;
            pub const CborKnownTags_CborRegularExpressionTag: CborKnownTags =
                35;
            pub const CborKnownTags_CborMimeMessageTag: CborKnownTags = 36;
            pub const CborKnownTags_CborSignatureTag: CborKnownTags = 55799;
            pub type CborKnownTags = u32;
            pub const CborError_CborNoError: CborError = 0;
            pub const CborError_CborUnknownError: CborError = 1;
            pub const CborError_CborErrorUnknownLength: CborError = 2;
            pub const CborError_CborErrorAdvancePastEOF: CborError = 3;
            pub const CborError_CborErrorIO: CborError = 4;
            pub const CborError_CborErrorGarbageAtEnd: CborError = 256;
            pub const CborError_CborErrorUnexpectedEOF: CborError = 257;
            pub const CborError_CborErrorUnexpectedBreak: CborError = 258;
            pub const CborError_CborErrorUnknownType: CborError = 259;
            pub const CborError_CborErrorIllegalType: CborError = 260;
            pub const CborError_CborErrorIllegalNumber: CborError = 261;
            pub const CborError_CborErrorIllegalSimpleType: CborError = 262;
            pub const CborError_CborErrorUnknownSimpleType: CborError = 512;
            pub const CborError_CborErrorUnknownTag: CborError = 513;
            pub const CborError_CborErrorInappropriateTagForType: CborError =
                514;
            pub const CborError_CborErrorDuplicateObjectKeys: CborError = 515;
            pub const CborError_CborErrorInvalidUtf8TextString: CborError =
                516;
            pub const CborError_CborErrorTooManyItems: CborError = 768;
            pub const CborError_CborErrorTooFewItems: CborError = 769;
            pub const CborError_CborErrorDataTooLarge: CborError = 1024;
            pub const CborError_CborErrorNestingTooDeep: CborError = 1025;
            pub const CborError_CborErrorUnsupportedType: CborError = 1026;
            pub const CborError_CborErrorJsonObjectKeyIsAggregate: CborError =
                1027;
            pub const CborError_CborErrorJsonObjectKeyNotString: CborError =
                1028;
            pub const CborError_CborErrorJsonNotImplemented: CborError = 1029;
            pub const CborError_CborErrorOutOfMemory: CborError = 2147483648;
            pub const CborError_CborErrorInternalError: CborError =
                4294967295;
            pub type CborError = u32;
            extern "C" {
                pub fn cbor_error_string(error: CborError)
                 -> *const ::cty::c_char;
            }
            pub type cbor_encoder_write
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut cbor_encoder_writer,
                                                            data:
                                                                *const ::cty::c_char,
                                                            len: ::cty::c_int)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct cbor_encoder_writer {
                pub write: cbor_encoder_write,
                pub bytes_written: ::cty::c_int,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for cbor_encoder_writer {
                #[inline]
                fn default() -> cbor_encoder_writer {
                    cbor_encoder_writer{write:
                                            ::core::default::Default::default(),
                                        bytes_written:
                                            ::core::default::Default::default(),}
                }
            }
            #[repr(C)]
            pub struct cbor_iovec {
                pub iov_base: *mut ::cty::c_void,
                pub iov_len: usize,
            }
            impl Default for cbor_iovec {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct CborEncoder {
                pub writer: *mut cbor_encoder_writer,
                pub writer_arg: *mut ::cty::c_void,
                pub added: usize,
                pub flags: ::cty::c_int,
            }
            impl Default for CborEncoder {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc =
                      " Initializes a CborEncoder structure \\a encoder by pointing it to buffer \\a"]
                #[doc =
                      " buffer of size \\a size. The \\a flags field is currently unused and must be"]
                #[doc = " zero."]
                pub fn cbor_encoder_init(encoder: *mut CborEncoder,
                                         pwriter: *mut cbor_encoder_writer,
                                         flags: ::cty::c_int);
            }
            extern "C" {
                #[doc =
                      " Appends the unsigned 64-bit integer \\a value to the CBOR stream provided by"]
                #[doc = " \\a encoder."]
                #[doc = ""]
                #[doc = " \\sa cbor_encode_negative_int, cbor_encode_int"]
                pub fn cbor_encode_uint(encoder: *mut CborEncoder, value: u64)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the signed 64-bit integer \\a value to the CBOR stream provided by"]
                #[doc = " \\a encoder."]
                #[doc = ""]
                #[doc = " \\sa cbor_encode_negative_int, cbor_encode_uint"]
                pub fn cbor_encode_int(encoder: *mut CborEncoder, value: i64)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the negative 64-bit integer whose absolute value is \\a"]
                #[doc =
                      " absolute_value to the CBOR stream provided by \\a encoder."]
                #[doc = ""]
                #[doc = " \\sa cbor_encode_uint, cbor_encode_int"]
                pub fn cbor_encode_negative_int(encoder: *mut CborEncoder,
                                                absolute_value: u64)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the CBOR Simple Type of value \\a value to the CBOR stream provided by"]
                #[doc = " \\a encoder."]
                #[doc = ""]
                #[doc =
                      " This function may return error CborErrorIllegalSimpleType if the \\a value"]
                #[doc =
                      " variable contains a number that is not a valid simple type."]
                pub fn cbor_encode_simple_value(encoder: *mut CborEncoder,
                                                value: u8) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the CBOR tag \\a tag to the CBOR stream provided by \\a encoder."]
                #[doc = ""]
                #[doc = " \\sa CborTag"]
                pub fn cbor_encode_tag(encoder: *mut CborEncoder,
                                       tag: CborTag) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the byte string \\a string of length \\a length to the CBOR stream"]
                #[doc =
                      " provided by \\a encoder. CBOR byte strings are arbitrary raw data."]
                #[doc = ""]
                #[doc =
                      " \\sa cbor_encode_text_stringz, cbor_encode_text_string"]
                pub fn cbor_encode_text_string(encoder: *mut CborEncoder,
                                               string: *const ::cty::c_char,
                                               length: usize) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the text string \\a string of length \\a length to the CBOR stream"]
                #[doc =
                      " provided by \\a encoder. CBOR requires that \\a string be valid UTF-8, but"]
                #[doc = " TinyCBOR makes no verification of correctness."]
                #[doc = ""]
                #[doc =
                      " \\sa CborError cbor_encode_text_stringz, cbor_encode_byte_string"]
                pub fn cbor_encode_byte_string(encoder: *mut CborEncoder,
                                               string: *const u8,
                                               length: usize) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the byte string passed as \\a iov and \\a iov_len to the CBOR"]
                #[doc =
                      " stream provided by \\a encoder. CBOR byte strings are arbitrary raw data."]
                #[doc = ""]
                #[doc =
                      " \\sa CborError cbor_encode_text_stringz, cbor_encode_byte_string"]
                pub fn cbor_encode_byte_iovec(encoder: *mut CborEncoder,
                                              iov: *const cbor_iovec,
                                              iov_len: ::cty::c_int)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Appends the floating-point value of type \\a fpType and pointed to by \\a"]
                #[doc =
                      " value to the CBOR stream provided by \\a encoder. The value of \\a fpType must"]
                #[doc =
                      " be one of CborHalfFloatType, CborFloatType or CborDoubleType, otherwise the"]
                #[doc = " behavior of this function is undefined."]
                #[doc = ""]
                #[doc =
                      " This function is useful for code that needs to pass through floating point"]
                #[doc =
                      " values but does not wish to have the actual floating-point code."]
                #[doc = ""]
                #[doc =
                      " \\sa cbor_encode_half_float, cbor_encode_float, cbor_encode_double"]
                pub fn cbor_encode_floating_point(encoder: *mut CborEncoder,
                                                  fpType: CborType,
                                                  value: *const ::cty::c_void)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Creates a CBOR array in the CBOR stream provided by \\a encoder and"]
                #[doc =
                      " initializes \\a arrayEncoder so that items can be added to the array using"]
                #[doc =
                      " the CborEncoder functions. The array must be terminated by calling either"]
                #[doc =
                      " cbor_encoder_close_container() or cbor_encoder_close_container_checked()"]
                #[doc =
                      " with the same \\a encoder and \\a arrayEncoder parameters."]
                #[doc = ""]
                #[doc =
                      " The number of items inserted into the array must be exactly \\a length items,"]
                #[doc =
                      " otherwise the stream is invalid. If the number of items is not known when"]
                #[doc =
                      " creating the array, the constant \\ref CborIndefiniteLength may be passed as"]
                #[doc = " length instead."]
                #[doc = ""]
                #[doc = " \\sa cbor_encoder_create_map"]
                pub fn cbor_encoder_create_array(encoder: *mut CborEncoder,
                                                 arrayEncoder:
                                                     *mut CborEncoder,
                                                 length: usize) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Creates a CBOR map in the CBOR stream provided by \\a encoder and"]
                #[doc =
                      " initializes \\a mapEncoder so that items can be added to the map using"]
                #[doc =
                      " the CborEncoder functions. The map must be terminated by calling either"]
                #[doc =
                      " cbor_encoder_close_container() or cbor_encoder_close_container_checked()"]
                #[doc =
                      " with the same \\a encoder and \\a mapEncoder parameters."]
                #[doc = ""]
                #[doc =
                      " The number of pair of items inserted into the map must be exactly \\a length"]
                #[doc =
                      " items, otherwise the stream is invalid. If the number of items is not known"]
                #[doc =
                      " when creating the map, the constant \\ref CborIndefiniteLength may be passed as"]
                #[doc = " length instead."]
                #[doc = ""]
                #[doc =
                      " \\b{Implementation limitation:} TinyCBOR cannot encode more than SIZE_MAX/2"]
                #[doc =
                      " key-value pairs in the stream. If the length \\a length is larger than this"]
                #[doc =
                      " value, this function returns error CborErrorDataTooLarge."]
                #[doc = ""]
                #[doc = " \\sa cbor_encoder_create_array"]
                pub fn cbor_encoder_create_map(encoder: *mut CborEncoder,
                                               mapEncoder: *mut CborEncoder,
                                               length: usize) -> CborError;
            }
            extern "C" {
                #[doc =
                      " Creates a indefinite-length byte string in the CBOR stream provided by"]
                #[doc =
                      " \\a encoder and initializes \\a stringEncoder so that chunks of original string"]
                #[doc =
                      " can be added using the CborEncoder functions. The string must be terminated by"]
                #[doc =
                      " calling cbor_encoder_close_container() with the same \\a encoder and"]
                #[doc = " \\a stringEncoder parameters."]
                #[doc = ""]
                #[doc = " \\sa cbor_encoder_create_array"]
                pub fn cbor_encoder_create_indef_byte_string(encoder:
                                                                 *mut CborEncoder,
                                                             stringEncoder:
                                                                 *mut CborEncoder)
                 -> CborError;
            }
            extern "C" {
                #[doc =
                      " Closes the CBOR container (array, map or indefinite-length string) provided"]
                #[doc =
                      " by \\a containerEncoder and updates the CBOR stream provided by \\a encoder."]
                #[doc =
                      " Both parameters must be the same as were passed to cbor_encoder_create_array() or"]
                #[doc =
                      " cbor_encoder_create_map() or cbor_encoder_create_indef_byte_string()."]
                #[doc = ""]
                #[doc =
                      " This function does not verify that the number of items (or pair of items, in"]
                #[doc =
                      " the case of a map) was correct. To execute that verification, call"]
                #[doc = " cbor_encoder_close_container_checked() instead."]
                #[doc = ""]
                #[doc =
                      " \\sa cbor_encoder_create_array(), cbor_encoder_create_map()"]
                pub fn cbor_encoder_close_container(encoder: *mut CborEncoder,
                                                    containerEncoder:
                                                        *const CborEncoder)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_encoder_close_container_checked(encoder:
                                                                *mut CborEncoder,
                                                            containerEncoder:
                                                                *const CborEncoder)
                 -> CborError;
            }
            pub const CborParserIteratorFlags_CborIteratorFlag_IntegerValueTooLarge:
                      CborParserIteratorFlags =
                1;
            pub const CborParserIteratorFlags_CborIteratorFlag_NegativeInteger:
                      CborParserIteratorFlags =
                2;
            pub const CborParserIteratorFlags_CborIteratorFlag_UnknownLength:
                      CborParserIteratorFlags =
                4;
            pub const CborParserIteratorFlags_CborIteratorFlag_ContainerIsMap:
                      CborParserIteratorFlags =
                32;
            pub type CborParserIteratorFlags = u32;
            pub type cbor_reader_get8
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            offset:
                                                                ::cty::c_int)
                                           -> u8>;
            pub type cbor_reader_get16
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            offset:
                                                                ::cty::c_int)
                                           -> u16>;
            pub type cbor_reader_get32
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            offset:
                                                                ::cty::c_int)
                                           -> u32>;
            pub type cbor_reader_get64
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            offset:
                                                                ::cty::c_int)
                                           -> u64>;
            pub type cbor_memcmp
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            buf:
                                                                *mut ::cty::c_char,
                                                            offset:
                                                                ::cty::c_int,
                                                            len: usize)
                                           -> usize>;
            pub type cbor_memcpy
                =
                ::core::option::Option<unsafe extern "C" fn(d:
                                                                *mut cbor_decoder_reader,
                                                            buf:
                                                                *mut ::cty::c_char,
                                                            offset:
                                                                ::cty::c_int,
                                                            len: usize)
                                           -> usize>;
            #[repr(C)]
            pub struct cbor_decoder_reader {
                pub get8: cbor_reader_get8,
                pub get16: cbor_reader_get16,
                pub get32: cbor_reader_get32,
                pub get64: cbor_reader_get64,
                pub cmp: cbor_memcmp,
                pub cpy: cbor_memcpy,
                pub message_size: usize,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for cbor_decoder_reader {
                #[inline]
                fn default() -> cbor_decoder_reader {
                    cbor_decoder_reader{get8:
                                            ::core::default::Default::default(),
                                        get16:
                                            ::core::default::Default::default(),
                                        get32:
                                            ::core::default::Default::default(),
                                        get64:
                                            ::core::default::Default::default(),
                                        cmp:
                                            ::core::default::Default::default(),
                                        cpy:
                                            ::core::default::Default::default(),
                                        message_size:
                                            ::core::default::Default::default(),}
                }
            }
            #[repr(C)]
            pub struct CborParser {
                pub d: *mut cbor_decoder_reader,
                pub end: ::cty::c_int,
                pub flags: ::cty::c_int,
            }
            impl Default for CborParser {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct CborValue {
                pub parser: *const CborParser,
                pub offset: ::cty::c_int,
                pub remaining: u32,
                pub extra: u16,
                pub type_: u8,
                pub flags: u8,
            }
            impl Default for CborValue {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                pub fn cbor_parser_init(d: *mut cbor_decoder_reader,
                                        flags: ::cty::c_int,
                                        parser: *mut CborParser,
                                        it: *mut CborValue) -> CborError;
            }
            extern "C" {
                pub fn cbor_value_advance_fixed(it: *mut CborValue)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_advance(it: *mut CborValue) -> CborError;
            }
            extern "C" {
                pub fn cbor_value_enter_container(it: *const CborValue,
                                                  recursed: *mut CborValue)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_leave_container(it: *mut CborValue,
                                                  recursed: *const CborValue)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_get_int64_checked(value: *const CborValue,
                                                    result: *mut i64)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_get_int_checked(value: *const CborValue,
                                                  result: *mut ::cty::c_int)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_skip_tag(it: *mut CborValue) -> CborError;
            }
            extern "C" {
                pub fn cbor_value_calculate_string_length(value:
                                                              *const CborValue,
                                                          length: *mut usize)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_text_string_equals(value: *const CborValue,
                                                     string:
                                                         *const ::cty::c_char,
                                                     result: *mut bool)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_map_find_value(map: *const CborValue,
                                                 string: *const ::cty::c_char,
                                                 element: *mut CborValue)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_get_half_float(value: *const CborValue,
                                                 result: *mut ::cty::c_void)
                 -> CborError;
            }
            extern "C" {
                pub fn cbor_value_to_pretty_advance(out: *mut FILE,
                                                    value: *mut CborValue)
                 -> CborError;
            }
            pub const CborMajorTypes_UnsignedIntegerType: CborMajorTypes = 0;
            pub const CborMajorTypes_NegativeIntegerType: CborMajorTypes = 1;
            pub const CborMajorTypes_ByteStringType: CborMajorTypes = 2;
            pub const CborMajorTypes_TextStringType: CborMajorTypes = 3;
            pub const CborMajorTypes_ArrayType: CborMajorTypes = 4;
            pub const CborMajorTypes_MapType: CborMajorTypes = 5;
            pub const CborMajorTypes_TagType: CborMajorTypes = 6;
            pub const CborMajorTypes_SimpleTypesType: CborMajorTypes = 7;
            pub type CborMajorTypes = u32;
            pub const CborSimpleTypes_FalseValue: CborSimpleTypes = 20;
            pub const CborSimpleTypes_TrueValue: CborSimpleTypes = 21;
            pub const CborSimpleTypes_NullValue: CborSimpleTypes = 22;
            pub const CborSimpleTypes_UndefinedValue: CborSimpleTypes = 23;
            pub const CborSimpleTypes_SimpleTypeInNextByte: CborSimpleTypes =
                24;
            pub const CborSimpleTypes_HalfPrecisionFloat: CborSimpleTypes =
                25;
            pub const CborSimpleTypes_SinglePrecisionFloat: CborSimpleTypes =
                26;
            pub const CborSimpleTypes_DoublePrecisionFloat: CborSimpleTypes =
                27;
            pub const CborSimpleTypes_Break: CborSimpleTypes = 31;
            pub type CborSimpleTypes = u32;
        }
        pub mod json_context {
            //! JSON encoder state used by CoAP JSON encoding macros
            use cstr_core::CStr;
            use cty::*;
            /// Global instance that contains the current state of the JSON encoder. Only 1 encoding task is supported at a time.
            pub static mut JSON_CONTEXT: JsonContext =
                unsafe {
                    ::core::mem::transmute::<[u8; ::core::mem::size_of::<JsonContext>()],
                                             JsonContext>([0;
                                                              ::core::mem::size_of::<JsonContext>()])
                };
            /// JSON encoder state. Buffers the next key and value to be encoded.
            pub struct JsonContext {
                /// Static buffer for the key to be encoded. Will be passed to Mynewt JSON encoder API.  Always null-terminated.
                key_buffer: [u8; JSON_KEY_SIZE],
                /// Static buffer for the string value to be encoded. Will be passed to Mynewt JSON encoder API.  Always null-terminated.
                value_buffer: [u8; JSON_VALUE_SIZE],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for JsonContext {
                #[inline]
                fn default() -> JsonContext {
                    JsonContext{key_buffer:
                                    ::core::default::Default::default(),
                                value_buffer:
                                    ::core::default::Default::default(),}
                }
            }
            /// Size of the static key buffer
            const JSON_KEY_SIZE: usize = 32;
            /// Size of the static value buffer
            const JSON_VALUE_SIZE: usize = 32;
            impl JsonContext {
                /// Given a key `s`, return a `*char` pointer that is null-terminated. Used for encoding JSON keys.
                /// If `s` is null-terminated, return it as a pointer. Else copy `s` to the static buffer,
                /// append null and return the buffer as a pointer.
                pub fn key_to_cstr(&mut self, s: &[u8]) -> *const c_char {
                    if s.last() == Some(&0) {
                        return s.as_ptr() as *const c_char;
                    }
                    if !(s.len() < JSON_KEY_SIZE) {
                        {
                            ::core::panicking::panic(&("assertion failed: s.len() < JSON_KEY_SIZE",
                                                       "src/mynewt/encoding/json_context.rs",
                                                       32u32, 9u32))
                        }
                    };
                    self.key_buffer[..s.len()].copy_from_slice(s);
                    self.key_buffer[s.len()] = 0;
                    self.key_buffer.as_ptr() as *const c_char
                }
                /// Given a value `s`, return a `*char` pointer that is null-terminated. Used for encoding JSON values.
                /// If `s` is null-terminated, return it as a pointer. Else copy `s` to the static buffer,
                /// append null and return the buffer as a pointer.
                pub fn value_to_cstr(&mut self, s: &[u8]) -> *const c_char {
                    if s.last() == Some(&0) {
                        return s.as_ptr() as *const c_char;
                    }
                    if !(s.len() < JSON_VALUE_SIZE) {
                        {
                            ::core::panicking::panic(&("assertion failed: s.len() < JSON_VALUE_SIZE",
                                                       "src/mynewt/encoding/json_context.rs",
                                                       45u32, 9u32))
                        }
                    };
                    self.value_buffer[..s.len()].copy_from_slice(s);
                    self.value_buffer[s.len()] = 0;
                    self.value_buffer.as_ptr() as *const c_char
                }
                /// Compute the byte length of the string in `s`.
                /// If `s` is null-terminated, return length of `s` - 1. Else return length of `s`.
                pub fn cstr_len(&self, s: &[u8]) -> usize {
                    if s.last() == Some(&0) { return s.len() - 1; }
                    s.len()
                }
                /// Return the global CBOR encoder
                pub fn global_encoder(&self)
                 -> *mut super::tinycbor::CborEncoder {
                    unsafe { &mut super::super::g_encoder }
                }
                /// TODO: Return the CBOR encoder for the current map or array
                pub fn encoder(&self, _parent: &str, _child: &str)
                 -> *mut super::tinycbor::CborEncoder {
                    unsafe { &mut super::super::root_map }
                }
                /// Fail the encoding with an error if `res` is non-zero.
                pub fn check_result(&self, res: u32) {
                    {
                        match (&res, &0) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                      "`,\n right: `",
                                                                                                      "`"],
                                                                                                    &match (&&*left_val,
                                                                                                            &&*right_val)
                                                                                                         {
                                                                                                         (arg0,
                                                                                                          arg1)
                                                                                                         =>
                                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                                       ::core::fmt::Debug::fmt),
                                                                                                          ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                                       ::core::fmt::Debug::fmt)],
                                                                                                     }),
                                                                     &("src/mynewt/encoding/json_context.rs",
                                                                       72u32,
                                                                       9u32))
                                    }
                                }
                            }
                        }
                    };
                }
                /// Fail the encoding with an error
                pub fn fail(&mut self, err: JsonError) {
                    {
                        match (&err, &JsonError::OK) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    {
                                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                                      "`,\n right: `",
                                                                                                      "`"],
                                                                                                    &match (&&*left_val,
                                                                                                            &&*right_val)
                                                                                                         {
                                                                                                         (arg0,
                                                                                                          arg1)
                                                                                                         =>
                                                                                                         [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                                       ::core::fmt::Debug::fmt),
                                                                                                          ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                                       ::core::fmt::Debug::fmt)],
                                                                                                     }),
                                                                     &("src/mynewt/encoding/json_context.rs",
                                                                       77u32,
                                                                       9u32))
                                    }
                                }
                            }
                        }
                    };
                }
                /// Cast itself as a `*mut c_void`
                pub fn to_void_ptr(&mut self) -> *mut c_void {
                    let ptr: *mut JsonContext = self;
                    ptr as *mut c_void
                }
            }
            /// Error codes for JSON encoding failure
            pub enum JsonError {

                /// No error
                OK = 0,

                /// Encoded value is not unsigned integer
                VALUE_NOT_UINT = 1,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for JsonError {
                fn fmt(&self, f: &mut ::core::fmt::Formatter)
                 -> ::core::fmt::Result {
                    match (&*self,) {
                        (&JsonError::OK,) => {
                            let mut debug_trait_builder = f.debug_tuple("OK");
                            debug_trait_builder.finish()
                        }
                        (&JsonError::VALUE_NOT_UINT,) => {
                            let mut debug_trait_builder =
                                f.debug_tuple("VALUE_NOT_UINT");
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for JsonError {
                #[inline]
                fn eq(&self, other: &JsonError) -> bool {
                    {
                        let __self_vi =
                            unsafe {
                                ::core::intrinsics::discriminant_value(&*self)
                            } as isize;
                        let __arg_1_vi =
                            unsafe {
                                ::core::intrinsics::discriminant_value(&*other)
                            } as isize;
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*other) { _ => true, }
                        } else { false }
                    }
                }
            }
            /// Convert the type to array of bytes that may or may not end with null
            pub trait ToBytesOptionalNull {
                /// Convert the type to array of bytes that may or may not end with null
                fn to_bytes_optional_nul(&self)
                -> &[u8];
            }
            /// Convert the type to array of bytes that may or may not end with null
            impl ToBytesOptionalNull for [u8] {
                /// Convert the type to array of bytes that may or may not end with null
                fn to_bytes_optional_nul(&self) -> &[u8] { self }
            }
            /// Convert the type to array of bytes that may or may not end with null
            impl ToBytesOptionalNull for str {
                /// Convert the type to array of bytes that may or may not end with null
                fn to_bytes_optional_nul(&self) -> &[u8] { self.as_bytes() }
            }
            /// Convert the type to array of bytes that may or may not end with null
            impl ToBytesOptionalNull for &str {
                /// Convert the type to array of bytes that may or may not end with null
                fn to_bytes_optional_nul(&self) -> &[u8] { self.as_bytes() }
            }
            /// Convert the type to array of bytes that may or may not end with null. CStr always includes nulls.
            impl ToBytesOptionalNull for CStr {
                /// Convert the type to array of bytes that may or may not end with null. CStr always includes nulls.
                fn to_bytes_optional_nul(&self) -> &[u8] {
                    self.to_bytes_with_nul()
                }
            }
        }
    }
    pub mod kernel {
        //! Mynewt Kernel API for Rust
        /// Contains Rust bindings for Mynewt OS API `kernel/os`
        pub mod os {
            #[repr(C)]
            pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>,
                                                 [T; 0]);
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <T: ::core::default::Default> ::core::default::Default for
             __IncompleteArrayField<T> {
                #[inline]
                fn default() -> __IncompleteArrayField<T> {
                    __IncompleteArrayField(::core::default::Default::default(),
                                           ::core::default::Default::default())
                }
            }
            impl <T> __IncompleteArrayField<T> {
                #[inline]
                pub fn new() -> Self {
                    __IncompleteArrayField(::core::marker::PhantomData, [])
                }
                #[inline]
                pub unsafe fn as_ptr(&self) -> *const T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_slice(&self, len: usize) -> &[T] {
                    ::core::slice::from_raw_parts(self.as_ptr(), len)
                }
                #[inline]
                pub unsafe fn as_mut_slice(&mut self, len: usize)
                 -> &mut [T] {
                    ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
                }
            }
            impl <T> ::core::fmt::Debug for __IncompleteArrayField<T> {
                fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
                 -> ::core::fmt::Result {
                    fmt.write_str("__IncompleteArrayField")
                }
            }
            impl <T> ::core::clone::Clone for __IncompleteArrayField<T> {
                #[inline]
                fn clone(&self) -> Self { Self::new() }
            }
            pub const OS_WAIT_FOREVER: i32 = -1;
            pub const OS_IDLE_PRIO: u32 = 255;
            pub const OS_TICKS_PER_SEC: u32 = 1000;
            pub const OS_SANITY_STACK_SIZE: u32 = 64;
            pub const OS_IDLE_STACK_SIZE: u32 = 64;
            pub const OS_STACK_PATTERN: u32 = 3735928559;
            pub const OS_ALIGNMENT: u32 = 4;
            pub const OS_STACK_ALIGNMENT: u32 = 8;
            pub const OS_TIME_MAX: u32 = 4294967295;
            pub const OS_STIME_MAX: u32 = 2147483647;
            pub const OS_TIMEOUT_NEVER: u32 = 4294967295;
            pub const OS_DEV_INIT_PRIMARY: u32 = 1;
            pub const OS_DEV_INIT_SECONDARY: u32 = 2;
            pub const OS_DEV_INIT_KERNEL: u32 = 3;
            pub const OS_DEV_INIT_F_CRITICAL: u32 = 1;
            pub const OS_DEV_INIT_PRIO_DEFAULT: u32 = 255;
            pub const OS_DEV_F_STATUS_READY: u32 = 1;
            pub const OS_DEV_F_STATUS_OPEN: u32 = 2;
            pub const OS_DEV_F_STATUS_SUSPENDED: u32 = 4;
            pub const OS_DEV_F_INIT_CRITICAL: u32 = 8;
            pub const OS_MEMPOOL_F_EXT: u32 = 1;
            pub const OS_MEMPOOL_INFO_NAME_LEN: u32 = 32;
            pub const OS_TASK_PRI_HIGHEST: u32 = 0;
            pub const OS_TASK_PRI_LOWEST: u32 = 255;
            pub const OS_TASK_FLAG_NO_TIMEOUT: u32 = 1;
            pub const OS_TASK_FLAG_SEM_WAIT: u32 = 2;
            pub const OS_TASK_FLAG_MUTEX_WAIT: u32 = 4;
            pub const OS_TASK_FLAG_EVQ_WAIT: u32 = 8;
            pub const OS_TASK_MAX_NAME_LEN: u32 = 32;
            pub const OS_TRACE_ID_EVENTQ_PUT: u32 = 40;
            pub const OS_TRACE_ID_EVENTQ_GET_NO_WAIT: u32 = 41;
            pub const OS_TRACE_ID_EVENTQ_GET: u32 = 42;
            pub const OS_TRACE_ID_EVENTQ_REMOVE: u32 = 43;
            pub const OS_TRACE_ID_EVENTQ_POLL_0TIMO: u32 = 44;
            pub const OS_TRACE_ID_EVENTQ_POLL: u32 = 45;
            pub const OS_TRACE_ID_MUTEX_INIT: u32 = 50;
            pub const OS_TRACE_ID_MUTEX_RELEASE: u32 = 51;
            pub const OS_TRACE_ID_MUTEX_PEND: u32 = 52;
            pub const OS_TRACE_ID_SEM_INIT: u32 = 60;
            pub const OS_TRACE_ID_SEM_RELEASE: u32 = 61;
            pub const OS_TRACE_ID_SEM_PEND: u32 = 62;
            pub const OS_TRACE_ID_CALLOUT_INIT: u32 = 70;
            pub const OS_TRACE_ID_CALLOUT_STOP: u32 = 71;
            pub const OS_TRACE_ID_CALLOUT_RESET: u32 = 72;
            pub const OS_TRACE_ID_CALLOUT_TICK: u32 = 73;
            pub const OS_TRACE_ID_MEMBLOCK_GET: u32 = 80;
            pub const OS_TRACE_ID_MEMBLOCK_PUT_FROM_CB: u32 = 81;
            pub const OS_TRACE_ID_MEMBLOCK_PUT: u32 = 82;
            pub const OS_TRACE_ID_MBUF_GET: u32 = 90;
            pub const OS_TRACE_ID_MBUF_GET_PKTHDR: u32 = 91;
            pub const OS_TRACE_ID_MBUF_FREE: u32 = 92;
            pub const OS_TRACE_ID_MBUF_FREE_CHAIN: u32 = 93;
            pub const SYS_EOK: u32 = 0;
            pub const SYS_ENOMEM: i32 = -1;
            pub const SYS_EINVAL: i32 = -2;
            pub const SYS_ETIMEOUT: i32 = -3;
            pub const SYS_ENOENT: i32 = -4;
            pub const SYS_EIO: i32 = -5;
            pub const SYS_EAGAIN: i32 = -6;
            pub const SYS_EACCES: i32 = -7;
            pub const SYS_EBUSY: i32 = -8;
            pub const SYS_ENODEV: i32 = -9;
            pub const SYS_ERANGE: i32 = -10;
            pub const SYS_EALREADY: i32 = -11;
            pub const SYS_ENOTSUP: i32 = -12;
            pub const SYS_EUNKNOWN: i32 = -13;
            pub const SYS_EREMOTEIO: i32 = -14;
            pub const SYS_EDONE: i32 = -15;
            pub const SYS_EPERUSER: i32 = -65535;
            pub const OS_RUN_PRIV: u32 = 0;
            pub const OS_RUN_UNPRIV: u32 = 1;
            pub type __uint8_t = ::cty::c_uchar;
            pub type __int16_t = ::cty::c_short;
            pub type __uint16_t = ::cty::c_ushort;
            pub type __int32_t = ::cty::c_long;
            pub type __uint32_t = ::cty::c_ulong;
            pub type __int64_t = ::cty::c_longlong;
            extern "C" {
                pub fn os_info_init() -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_init_idle_task();
            }
            extern "C" {
                #[doc = " Check whether or not the OS has been started."]
                #[doc = ""]
                #[doc =
                      " Return: 1 if the OS has been started and 0 if it has not yet been started."]
                pub fn os_started() -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Initialize the OS, including memory areas and housekeeping functions."]
                #[doc =
                      " This calls into the architecture specific OS initialization."]
                #[doc = ""]
                #[doc =
                      " - __`fn`__: The system \"main\" function to start the main task with."]
                pub fn os_init(fn_:
                                   ::core::option::Option<unsafe extern "C" fn(argc:
                                                                                   ::cty::c_int,
                                                                               argv:
                                                                                   *mut *mut ::cty::c_char)
                                                              ->
                                                                  ::cty::c_int>);
            }
            extern "C" {
                #[doc = " Start the OS and begin processing."]
                pub fn os_start();
            }
            extern "C" {
                #[doc = " Reboots the system."]
                pub fn os_reboot(reason: ::cty::c_int);
            }
            extern "C" {
                #[doc =
                      " Performs a system reset.  This is typically done at the end of a reboot"]
                #[doc = " procedure."]
                pub fn os_system_reset();
            }
            pub type os_sr_t = u32;
            pub type os_stack_t = u32;
            pub const os_error_OS_OK: os_error = 0;
            pub const os_error_OS_ENOMEM: os_error = 1;
            pub const os_error_OS_EINVAL: os_error = 2;
            pub const os_error_OS_INVALID_PARM: os_error = 3;
            pub const os_error_OS_MEM_NOT_ALIGNED: os_error = 4;
            pub const os_error_OS_BAD_MUTEX: os_error = 5;
            pub const os_error_OS_TIMEOUT: os_error = 6;
            pub const os_error_OS_ERR_IN_ISR: os_error = 7;
            pub const os_error_OS_ERR_PRIV: os_error = 8;
            pub const os_error_OS_NOT_STARTED: os_error = 9;
            pub const os_error_OS_ENOENT: os_error = 10;
            pub const os_error_OS_EBUSY: os_error = 11;
            pub const os_error_OS_ERROR: os_error = 12;
            pub type os_error = u32;
            pub use self::os_error as os_error_t;
            #[repr(C)]
            pub struct os_stack {
                _unused: [u8; 0],
            }
            extern "C" {
                pub fn os_arch_task_stack_init(arg1: *mut os_task,
                                               arg2: *mut os_stack_t,
                                               arg3: ::cty::c_int)
                 -> *mut os_stack_t;
            }
            extern "C" {
                pub fn os_arch_ctx_sw(arg1: *mut os_task);
            }
            extern "C" {
                pub fn os_arch_save_sr() -> os_sr_t;
            }
            extern "C" {
                pub fn os_arch_restore_sr(arg1: os_sr_t);
            }
            extern "C" {
                pub fn os_arch_in_critical() -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_arch_init();
            }
            extern "C" {
                pub fn os_arch_start() -> u32;
            }
            extern "C" {
                pub fn os_arch_os_init() -> os_error_t;
            }
            extern "C" {
                pub fn os_arch_os_start() -> os_error_t;
            }
            extern "C" {
                pub fn os_set_env(arg1: *mut os_stack_t);
            }
            extern "C" {
                pub fn os_arch_init_task_stack(sf: *mut os_stack_t);
            }
            extern "C" {
                pub fn os_default_irq_asm();
            }
            pub type os_time_t = u32;
            pub type os_stime_t = i32;
            extern "C" {
                #[doc = " Get the current OS time in ticks"]
                #[doc = ""]
                #[doc = " Return: OS time in ticks"]
                pub fn os_time_get() -> os_time_t;
            }
            extern "C" {
                #[doc = " Move OS time forward ticks."]
                #[doc = ""]
                #[doc =
                      " - __`ticks`__: The number of ticks to move time forward."]
                pub fn os_time_advance(ticks: ::cty::c_int);
            }
            extern "C" {
                #[doc =
                      " Puts the current task to sleep for the specified number of os ticks. There"]
                #[doc = " is no delay if ticks is 0."]
                #[doc = ""]
                #[doc =
                      " - __`osticks`__: Number of ticks to delay (0 means no delay)."]
                pub fn os_time_delay(osticks: os_time_t);
            }
            #[doc =
                  " Structure representing time since Jan 1 1970 with microsecond"]
            #[doc = " granularity"]
            #[repr(C)]
            pub struct os_timeval {
                pub tv_sec: i64,
                pub tv_usec: i32,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for os_timeval {
                #[inline]
                fn default() -> os_timeval {
                    os_timeval{tv_sec: ::core::default::Default::default(),
                               tv_usec: ::core::default::Default::default(),}
                }
            }
            #[doc = " Structure representing a timezone offset"]
            #[repr(C)]
            pub struct os_timezone {
                #[doc = " Minutes west of GMT"]
                pub tz_minuteswest: i16,
                #[doc = " Daylight savings time correction (if any)"]
                pub tz_dsttime: i16,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for os_timezone {
                #[inline]
                fn default() -> os_timezone {
                    os_timezone{tz_minuteswest:
                                    ::core::default::Default::default(),
                                tz_dsttime:
                                    ::core::default::Default::default(),}
                }
            }
            #[doc =
                  " Represents a time change.  Passed to time change listeners when the current"]
            #[doc = " time-of-day is set."]
            #[repr(C)]
            pub struct os_time_change_info {
                #[doc = " UTC time prior to change."]
                pub tci_prev_tv: *const os_timeval,
                #[doc = " Time zone prior to change."]
                pub tci_prev_tz: *const os_timezone,
                #[doc = " UTC time after change."]
                pub tci_cur_tv: *const os_timeval,
                #[doc = " Time zone after change."]
                pub tci_cur_tz: *const os_timezone,
                pub tci_newly_synced: bool,
            }
            impl Default for os_time_change_info {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Callback that is executed when the time-of-day is set."]
            #[doc = ""]
            #[doc =
                  " - __`info`__:                  Describes the time change that just occurred."]
            #[doc =
                  " - __`arg`__:                   Optional argument correponding to listener."]
            pub type os_time_change_fn
                =
                ::core::option::Option<unsafe extern "C" fn(info:
                                                                *const os_time_change_info,
                                                            arg:
                                                                *mut ::cty::c_void)>;
            #[doc =
                  " Time change listener.  Notified when the time-of-day is set."]
            #[repr(C)]
            pub struct os_time_change_listener {
                #[doc = " Public."]
                pub tcl_fn: os_time_change_fn,
                pub tcl_arg: *mut ::cty::c_void,
                pub tcl_next: os_time_change_listener__bindgen_ty_1,
            }
            #[doc = " Internal."]
            #[repr(C)]
            pub struct os_time_change_listener__bindgen_ty_1 {
                pub stqe_next: *mut os_time_change_listener,
            }
            impl Default for os_time_change_listener__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_time_change_listener {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc =
                      " Set the time of day.  This does not modify os time, but rather just modifies"]
                #[doc =
                      " the offset by which we are tracking real time against os time.  This"]
                #[doc =
                      " function notifies all registered time change listeners."]
                #[doc = ""]
                #[doc =
                      " - __`utctime`__: A timeval representing the UTC time we are setting"]
                #[doc =
                      " - __`tz`__: The time-zone to apply against the utctime being set."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_settimeofday(utctime: *mut os_timeval,
                                       tz: *mut os_timezone) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Get the current time of day.  Returns the time of day in UTC"]
                #[doc =
                      " into the tv argument, and returns the timezone (if set) into"]
                #[doc = " tz."]
                #[doc = ""]
                #[doc =
                      " - __`tv`__: The structure to put the UTC time of day into"]
                #[doc =
                      " - __`tz`__: The structure to put the timezone information into"]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure"]
                pub fn os_gettimeofday(utctime: *mut os_timeval,
                                       tz: *mut os_timezone) -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_time_is_set() -> bool;
            }
            extern "C" {
                #[doc = " Get time since boot in microseconds."]
                #[doc = ""]
                #[doc = " Return: time since boot in microseconds"]
                pub fn os_get_uptime_usec() -> i64;
            }
            extern "C" {
                #[doc = " Get time since boot as os_timeval."]
                #[doc = ""]
                #[doc = " - __`tv`__: Structure to put the time since boot."]
                pub fn os_get_uptime(tvp: *mut os_timeval);
            }
            extern "C" {
                #[doc = " Converts milliseconds to OS ticks."]
                #[doc = ""]
                #[doc =
                      " - __`ms`__:                    The milliseconds input."]
                #[doc =
                      " - __`out_ticks`__:             The OS ticks output."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 on success; OS_EINVAL if the result is too"]
                #[doc =
                      "                                  large to fit in a uint32_t."]
                pub fn os_time_ms_to_ticks(ms: u32, out_ticks: *mut os_time_t)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Converts OS ticks to milliseconds."]
                #[doc = ""]
                #[doc = " - __`ticks`__:                 The OS ticks input."]
                #[doc =
                      " - __`out_ms`__:                The milliseconds output."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 on success; OS_EINVAL if the result is too"]
                #[doc =
                      "                                  large to fit in a uint32_t."]
                pub fn os_time_ticks_to_ms(ticks: os_time_t, out_ms: *mut u32)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Registers a time change listener.  Whenever the time is set, all registered"]
                #[doc =
                      " listeners are notified.  The provided pointer is added to an internal list,"]
                #[doc =
                      " so the listener's lifetime must extend indefinitely (or until the listener"]
                #[doc = " is removed)."]
                #[doc = ""]
                #[doc =
                      " NOTE: This function is not thread safe.  The following operations must be"]
                #[doc = " kept exclusive:"]
                #[doc = "     o Addition of listener"]
                #[doc = "     o Removal of listener"]
                #[doc = "     o Setting time"]
                #[doc = ""]
                #[doc =
                      " - __`listener`__:              The listener to register."]
                pub fn os_time_change_listen(listener:
                                                 *mut os_time_change_listener);
            }
            extern "C" {
                #[doc = " Unregisters a time change listener."]
                #[doc = ""]
                #[doc =
                      " NOTE: This function is not thread safe.  The following operations must be"]
                #[doc = " kept exclusive:"]
                #[doc = "     o Addition of listener"]
                #[doc = "     o Removal of listener"]
                #[doc = "     o Setting time"]
                #[doc = ""]
                #[doc =
                      " - __`listener`__:              The listener to unregister."]
                pub fn os_time_change_remove(listener:
                                                 *const os_time_change_listener)
                 -> ::cty::c_int;
            }
            pub type os_event_fn
                =
                ::core::option::Option<unsafe extern "C" fn(ev:
                                                                *mut os_event)>;
            #[doc =
                  " Structure representing an OS event.  OS events get placed onto the"]
            #[doc = " event queues and are consumed by tasks."]
            #[repr(C)]
            pub struct os_event {
                #[doc = " Whether this OS event is queued on an event queue."]
                pub ev_queued: u8,
                #[doc =
                      " Callback to call when the event is taken off of an event queue."]
                #[doc =
                      " APIs, except for os_eventq_run(), assume this callback will be called by"]
                #[doc = " the user."]
                pub ev_cb: os_event_fn,
                #[doc = " Argument to pass to the event queue callback."]
                pub ev_arg: *mut ::cty::c_void,
                pub ev_next: os_event__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_event__bindgen_ty_1 {
                pub stqe_next: *mut os_event,
            }
            impl Default for os_event__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_event {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_eventq {
                #[doc = " Pointer to task that \"owns\" this event queue."]
                pub evq_owner: *mut os_task,
                #[doc =
                      " Pointer to the task that is sleeping on this event queue, either NULL,"]
                #[doc = " or the owner task."]
                pub evq_task: *mut os_task,
                pub evq_list: os_eventq__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_eventq__bindgen_ty_1 {
                pub stqh_first: *mut os_event,
                pub stqh_last: *mut *mut os_event,
            }
            impl Default for os_eventq__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_eventq {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Initialize the event queue"]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to initialize"]
                pub fn os_eventq_init(arg1: *mut os_eventq);
            }
            extern "C" {
                #[doc = " Check whether the event queue is initialized."]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to check"]
                pub fn os_eventq_inited(evq: *const os_eventq)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Put an event on the event queue."]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to put an event on"]
                #[doc = " - __`ev`__: The event to put on the queue"]
                pub fn os_eventq_put(arg1: *mut os_eventq,
                                     arg2: *mut os_event);
            }
            extern "C" {
                #[doc =
                      " Poll an event from the event queue and return it immediately."]
                #[doc =
                      " If no event is available, don't block, just return NULL."]
                #[doc = ""]
                #[doc =
                      " Return: Event from the queue, or NULL if none available."]
                pub fn os_eventq_get_no_wait(evq: *mut os_eventq)
                 -> *mut os_event;
            }
            extern "C" {
                #[doc =
                      " Pull a single item from an event queue.  This function blocks until there"]
                #[doc = " is an item on the event queue to read."]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to pull an event from"]
                #[doc = ""]
                #[doc = " Return: The event from the queue"]
                pub fn os_eventq_get(arg1: *mut os_eventq) -> *mut os_event;
            }
            extern "C" {
                #[doc =
                      " Pull a single item off the event queue and call it's event"]
                #[doc = " callback."]
                #[doc = ""]
                #[doc = " - __`evq`__: The event queue to pull the item off."]
                pub fn os_eventq_run(evq: *mut os_eventq);
            }
            extern "C" {
                #[doc =
                      " Poll the list of event queues specified by the evq parameter"]
                #[doc =
                      " (size nevqs), and return the \"first\" event available on any of"]
                #[doc =
                      " the queues.  Event queues are searched in the order that they"]
                #[doc = " are passed in the array."]
                #[doc = ""]
                #[doc = " - __`evq`__: Array of event queues"]
                #[doc = " - __`nevqs`__: Number of event queues in evq"]
                #[doc =
                      " - __`timo`__: Timeout, forever if OS_WAIT_FOREVER is passed to poll."]
                #[doc = ""]
                #[doc = " Return: An event, or NULL if no events available"]
                pub fn os_eventq_poll(arg1: *mut *mut os_eventq,
                                      arg2: ::cty::c_int, arg3: os_time_t)
                 -> *mut os_event;
            }
            extern "C" {
                #[doc = " Remove an event from the queue."]
                #[doc = ""]
                #[doc =
                      " - __`evq`__: The event queue to remove the event from"]
                #[doc = " - __`ev`__:  The event to remove from the queue"]
                pub fn os_eventq_remove(arg1: *mut os_eventq,
                                        arg2: *mut os_event);
            }
            extern "C" {
                #[doc =
                      " Retrieves the default event queue processed by OS main task."]
                #[doc = ""]
                #[doc =
                      " Return:                      The default event queue."]
                pub fn os_eventq_dflt_get() -> *mut os_eventq;
            }
            extern "C" {
                #[doc = " @cond INTERNAL_HIDDEN"]
                #[doc = " [DEPRECATED]"]
                pub fn os_eventq_designate(dst: *mut *mut os_eventq,
                                           val: *mut os_eventq,
                                           start_ev: *mut os_event);
            }
            #[doc =
                  " Structure containing the definition of a callout, initialized"]
            #[doc = " by os_callout_init() and passed to callout functions."]
            #[repr(C)]
            pub struct os_callout {
                #[doc = " Event to post when the callout expires."]
                pub c_ev: os_event,
                #[doc = " Pointer to the event queue to post the event to"]
                pub c_evq: *mut os_eventq,
                #[doc =
                      " Number of ticks in the future to expire the callout"]
                pub c_ticks: os_time_t,
                pub c_next: os_callout__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_callout__bindgen_ty_1 {
                pub tqe_next: *mut os_callout,
                pub tqe_prev: *mut *mut os_callout,
            }
            impl Default for os_callout__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_callout {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " @cond INTERNAL_HIDDEN"]
            #[repr(C)]
            pub struct os_callout_list {
                pub tqh_first: *mut os_callout,
                pub tqh_last: *mut *mut os_callout,
            }
            impl Default for os_callout_list {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Initialize a callout."]
                #[doc = ""]
                #[doc =
                      " Callouts are used to schedule events in the future onto a task's event"]
                #[doc =
                      " queue.  Callout timers are scheduled using the os_callout_reset()"]
                #[doc =
                      " function.  When the timer expires, an event is posted to the event"]
                #[doc =
                      " queue specified in os_callout_init().  The event argument given here"]
                #[doc = " is posted in the ev_arg field of that event."]
                #[doc = ""]
                #[doc = " - __`c`__: The callout to initialize"]
                #[doc =
                      " - __`evq`__: The event queue to post an OS_EVENT_T_TIMER event to"]
                #[doc =
                      " - __`timo_func`__: The function to call on this callout for the host task"]
                #[doc =
                      "                  used to provide multiple timer events to a task"]
                #[doc = "                  (this can be NULL.)"]
                #[doc =
                      " - __`ev_arg`__: The argument to provide to the event when posting the"]
                #[doc = "               timer."]
                pub fn os_callout_init(cf: *mut os_callout,
                                       evq: *mut os_eventq,
                                       ev_cb: os_event_fn,
                                       ev_arg: *mut ::cty::c_void);
            }
            extern "C" {
                #[doc =
                      " Stop the callout from firing off, any pending events will be cleared."]
                #[doc = ""]
                #[doc = " - __`c`__: The callout to stop"]
                pub fn os_callout_stop(arg1: *mut os_callout);
            }
            extern "C" {
                #[doc = " Reset the callout to fire off in 'ticks' ticks."]
                #[doc = ""]
                #[doc = " - __`c`__: The callout to reset"]
                #[doc =
                      " - __`ticks`__: The number of ticks to wait before posting an event"]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure"]
                pub fn os_callout_reset(arg1: *mut os_callout,
                                        arg2: os_time_t) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Returns the number of ticks which remains to callout."]
                #[doc = ""]
                #[doc = " - __`c`__: The callout to check"]
                #[doc = " - __`now`__: The current time in OS ticks"]
                #[doc = ""]
                #[doc = " Return: Number of ticks to first pending callout"]
                pub fn os_callout_remaining_ticks(arg1: *mut os_callout,
                                                  arg2: os_time_t)
                 -> os_time_t;
            }
            extern "C" {
                #[doc = " @cond INTERNAL_HIDDEN"]
                pub fn os_callout_tick();
            }
            extern "C" {
                pub fn os_callout_wakeup_ticks(now: os_time_t) -> os_time_t;
            }
            pub type hal_timer_cb
                =
                ::core::option::Option<unsafe extern "C" fn(arg:
                                                                *mut ::cty::c_void)>;
            #[doc =
                  " The HAL timer structure. The user can declare as many of these structures"]
            #[doc =
                  " as desired. They are enqueued on a particular HW timer queue when the user"]
            #[doc =
                  " calls the :c:func:`hal_timer_start()` or :c:func:`hal_timer_start_at()` API."]
            #[doc =
                  " The user must have called :c:func:`hal_timer_set_cb()` before starting a"]
            #[doc = " timer."]
            #[doc = ""]
            #[doc =
                  " NOTE: the user should not have to modify/examine the contents of this"]
            #[doc = " structure; the hal timer API should be used."]
            #[repr(C)]
            pub struct hal_timer {
                #[doc = " Internal platform specific pointer"]
                pub bsp_timer: *mut ::cty::c_void,
                #[doc = " Callback function"]
                pub cb_func: hal_timer_cb,
                #[doc = " Callback argument"]
                pub cb_arg: *mut ::cty::c_void,
                #[doc = " Tick at which timer should expire"]
                pub expiry: u32,
                pub link: hal_timer__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct hal_timer__bindgen_ty_1 {
                pub tqe_next: *mut hal_timer,
                pub tqe_prev: *mut *mut hal_timer,
            }
            impl Default for hal_timer__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for hal_timer {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc =
                      " Initialize the cputime module. This must be called after os_init is called"]
                #[doc =
                      " and before any other timer API are used. This should be called only once"]
                #[doc =
                      " and should be called before the hardware timer is used."]
                #[doc = ""]
                #[doc =
                      " - __`clock_freq`__: The desired cputime frequency, in hertz (Hz)."]
                #[doc = ""]
                #[doc = " Return: int 0 on success; -1 on error."]
                pub fn os_cputime_init(clock_freq: u32) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Returns the low 32 bits of cputime."]
                #[doc = ""]
                #[doc = " Return: uint32_t The lower 32 bits of cputime"]
                pub fn os_cputime_get32() -> u32;
            }
            extern "C" {
                #[doc =
                      " Converts the given number of nanoseconds into cputime ticks."]
                #[doc = " Not defined if OS_CPUTIME_FREQ_PWR2 is defined."]
                #[doc = ""]
                #[doc =
                      " - __`usecs`__: The number of nanoseconds to convert to ticks"]
                #[doc = ""]
                #[doc =
                      " Return: uint32_t The number of ticks corresponding to 'nsecs'"]
                pub fn os_cputime_nsecs_to_ticks(nsecs: u32) -> u32;
            }
            extern "C" {
                #[doc =
                      " Convert the given number of ticks into nanoseconds."]
                #[doc = " Not defined if OS_CPUTIME_FREQ_PWR2 is defined."]
                #[doc = ""]
                #[doc =
                      " - __`ticks`__: The number of ticks to convert to nanoseconds."]
                #[doc = ""]
                #[doc =
                      " Return: uint32_t The number of nanoseconds corresponding to 'ticks'"]
                pub fn os_cputime_ticks_to_nsecs(ticks: u32) -> u32;
            }
            extern "C" {
                #[doc =
                      " Wait until 'nsecs' nanoseconds has elapsed. This is a blocking delay."]
                #[doc = " Not defined if OS_CPUTIME_FREQ_PWR2 is defined."]
                #[doc = ""]
                #[doc = ""]
                #[doc = " - __`nsecs`__: The number of nanoseconds to wait."]
                pub fn os_cputime_delay_nsecs(nsecs: u32);
            }
            extern "C" {
                #[doc =
                      " Wait until the number of ticks has elapsed. This is a blocking delay."]
                #[doc = ""]
                #[doc = " - __`ticks`__: The number of ticks to wait."]
                pub fn os_cputime_delay_ticks(ticks: u32);
            }
            extern "C" {
                #[doc =
                      " Wait until 'usecs' microseconds has elapsed. This is a blocking delay."]
                #[doc = ""]
                #[doc = " - __`usecs`__: The number of usecs to wait."]
                pub fn os_cputime_delay_usecs(usecs: u32);
            }
            extern "C" {
                #[doc = " Initialize a CPU timer, using the given HAL timer."]
                #[doc = ""]
                #[doc =
                      " - __`timer`__: The timer to initialize. Cannot be NULL."]
                #[doc =
                      " - __`fp`__:    The timer callback function. Cannot be NULL."]
                #[doc =
                      " - __`arg`__:   Pointer to data object to pass to timer."]
                pub fn os_cputime_timer_init(timer: *mut hal_timer,
                                             fp: hal_timer_cb,
                                             arg: *mut ::cty::c_void);
            }
            extern "C" {
                #[doc =
                      " Start a cputimer that will expire at 'cputime'. If cputime has already"]
                #[doc =
                      " passed, the timer callback will still be called (at interrupt context)."]
                #[doc = ""]
                #[doc =
                      " NOTE: This must be called when the timer is stopped."]
                #[doc = ""]
                #[doc =
                      " - __`timer`__:     Pointer to timer to start. Cannot be NULL."]
                #[doc =
                      " - __`cputime`__:   The cputime at which the timer should expire."]
                #[doc = ""]
                #[doc =
                      " Return: int 0 on success; EINVAL if timer already started or timer struct"]
                #[doc = "         invalid"]
                #[doc = ""]
                pub fn os_cputime_timer_start(timer: *mut hal_timer,
                                              cputime: u32) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Sets a cpu timer that will expire 'usecs' microseconds from the current"]
                #[doc = " cputime."]
                #[doc = ""]
                #[doc =
                      " NOTE: This must be called when the timer is stopped."]
                #[doc = ""]
                #[doc = " - __`timer`__: Pointer to timer. Cannot be NULL."]
                #[doc =
                      " - __`usecs`__: The number of usecs from now at which the timer will expire."]
                #[doc = ""]
                #[doc =
                      " Return: int 0 on success; EINVAL if timer already started or timer struct"]
                #[doc = "         invalid"]
                pub fn os_cputime_timer_relative(timer: *mut hal_timer,
                                                 usecs: u32) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Stops a cputimer from running. The timer is removed from the timer queue"]
                #[doc =
                      " and interrupts are disabled if no timers are left on the queue. Can be"]
                #[doc = " called even if timer is not running."]
                #[doc = ""]
                #[doc =
                      " - __`timer`__: Pointer to cputimer to stop. Cannot be NULL."]
                pub fn os_cputime_timer_stop(timer: *mut hal_timer);
            }
            #[doc = " Initialize a device."]
            #[doc = ""]
            #[doc = " - __`dev`__: The device to initialize."]
            #[doc =
                  " - __`arg`__: User defined argument to pass to the device initalization"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type os_dev_init_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            pub type os_dev_open_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2: u32,
                                                            arg3:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            pub type os_dev_suspend_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2: os_time_t,
                                                            arg3:
                                                                ::cty::c_int)
                                           -> ::cty::c_int>;
            pub type os_dev_resume_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev)
                                           -> ::cty::c_int>;
            pub type os_dev_close_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev)
                                           -> ::cty::c_int>;
            #[doc =
                  " Device handlers, implementers of device drivers should fill these"]
            #[doc = " out to control device operation."]
            #[repr(C)]
            pub struct os_dev_handlers {
                #[doc =
                      " Device open handler, called when the user opens the device."]
                #[doc =
                      " Any locking of the device should be done within the open handler."]
                pub od_open: os_dev_open_func_t,
                #[doc =
                      " Suspend handler, called when the device is being suspended."]
                #[doc =
                      " Up to the implementer to save device state before power down,"]
                #[doc =
                      " so that the device can be cleanly resumed -- or error out and"]
                #[doc = " delay suspension."]
                pub od_suspend: os_dev_suspend_func_t,
                #[doc =
                      " Resume handler, restores device state after a suspend operation."]
                pub od_resume: os_dev_resume_func_t,
                #[doc =
                      " Close handler, releases the device, including any locks that"]
                #[doc = " may have been taken by open()."]
                pub od_close: os_dev_close_func_t,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for os_dev_handlers {
                #[inline]
                fn default() -> os_dev_handlers {
                    os_dev_handlers{od_open:
                                        ::core::default::Default::default(),
                                    od_suspend:
                                        ::core::default::Default::default(),
                                    od_resume:
                                        ::core::default::Default::default(),
                                    od_close:
                                        ::core::default::Default::default(),}
                }
            }
            #[repr(C)]
            pub struct os_dev {
                #[doc =
                      " Device handlers.  Implementation of base device functions."]
                pub od_handlers: os_dev_handlers,
                #[doc = " Device initialization function."]
                pub od_init: os_dev_init_func_t,
                #[doc =
                      " Argument to pass to device initialization function."]
                pub od_init_arg: *mut ::cty::c_void,
                #[doc = " Stage during which to initialize this device."]
                pub od_stage: u8,
                #[doc =
                      " Priority within a given stage to initialize a device."]
                pub od_priority: u8,
                #[doc =
                      " Number of references to a device being open before marking"]
                #[doc = " the device closed."]
                pub od_open_ref: u8,
                #[doc = " Device flags."]
                pub od_flags: u8,
                #[doc = " Device name"]
                pub od_name: *const ::cty::c_char,
                pub od_next: os_dev__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_dev__bindgen_ty_1 {
                pub stqe_next: *mut os_dev,
            }
            impl Default for os_dev__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_dev {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Suspend the operation of the device."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to suspend."]
                #[doc =
                      " - __`suspend_t`__: When the device should be suspended."]
                #[doc =
                      " - __`force`__: Whether not the suspend operation can be overridden by the"]
                #[doc = "        device handler."]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub fn os_dev_suspend(dev: *mut os_dev, suspend_t: os_time_t,
                                      force: u8) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Resume the device operation."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to resume"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub fn os_dev_resume(dev: *mut os_dev) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Create a new device in the kernel."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to create."]
                #[doc = " - __`name`__: The name of the device to create."]
                #[doc =
                      " - __`stage`__: The stage to initialize that device to."]
                #[doc =
                      " - __`priority`__: The priority of initializing that device"]
                #[doc =
                      " - __`od_init`__: The initialization function to call for this"]
                #[doc = "                device."]
                #[doc =
                      " - __`arg`__: The argument to provide this device initialization"]
                #[doc = "            function."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_dev_create(dev: *mut os_dev,
                                     name: *const ::cty::c_char, stage: u8,
                                     priority: u8,
                                     od_init: os_dev_init_func_t,
                                     arg: *mut ::cty::c_void) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Lookup a device by name."]
                #[doc = ""]
                #[doc =
                      " WARNING: This should be called before any locking on the device is done, or"]
                #[doc =
                      " the device list itself is modified in any context.  There is no locking."]
                #[doc = ""]
                #[doc = " - __`name`__: The name of the device to look up."]
                #[doc = ""]
                #[doc =
                      " Return: A pointer to the device corresponding to name, or NULL if not found."]
                pub fn os_dev_lookup(name: *const ::cty::c_char)
                 -> *mut os_dev;
            }
            extern "C" {
                #[doc = " Initialize all devices for a given state."]
                #[doc = ""]
                #[doc = " - __`stage`__: The stage to initialize."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_dev_initialize_all(arg1: u8) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Suspend all devices."]
                #[doc = ""]
                #[doc =
                      " - __`suspend_t`__: The number of ticks to suspend this device for"]
                #[doc =
                      " - __`force`__: Whether or not to force suspending the device"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, or a non-zero error code if one of the devices"]
                #[doc = "                       returned it."]
                pub fn os_dev_suspend_all(arg1: os_time_t, arg2: u8)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Resume all the devices that were suspended."]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, -1 if any of the devices have failed to resume."]
                pub fn os_dev_resume_all() -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Open a device."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to open"]
                #[doc =
                      " - __`timo`__: The timeout to open the device, if not specified."]
                #[doc =
                      " - __`arg`__: The argument to the device open() call."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_dev_open(devname: *const ::cty::c_char, timo: u32,
                                   arg: *mut ::cty::c_void) -> *mut os_dev;
            }
            extern "C" {
                #[doc = " Close a device."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to close"]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_dev_close(dev: *mut os_dev) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Clears the device list.  This function does not close any devices or free"]
                #[doc =
                      " any resources; its purpose is to allow a full system reset between unit"]
                #[doc = " tests."]
                pub fn os_dev_reset();
            }
            extern "C" {
                #[doc =
                      " Walk through all devices, calling callback for every device."]
                #[doc = ""]
                #[doc = " - __`walk_func`__: Function to call"]
                #[doc = " @aparm arg       Argument to pass to walk_func"]
                pub fn os_dev_walk(walk_func:
                                       ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                                       *mut os_dev,
                                                                                   arg2:
                                                                                       *mut ::cty::c_void)
                                                                  ->
                                                                      ::cty::c_int>,
                                   arg: *mut ::cty::c_void);
            }
            extern "C" {
                #[doc =
                      " Operating system level malloc().   This ensures that a safe malloc occurs"]
                #[doc =
                      " within the context of the OS.  Depending on platform, the OS may rely on"]
                #[doc =
                      " libc's malloc() implementation, which is not guaranteed to be thread-safe."]
                #[doc = " This malloc() will always be thread-safe."]
                #[doc = ""]
                #[doc = " - __`size`__: The number of bytes to allocate"]
                #[doc = ""]
                #[doc = " Return: A pointer to the memory region allocated."]
                pub fn os_malloc(size: usize) -> *mut ::cty::c_void;
            }
            extern "C" {
                #[doc =
                      " Operating system level free().  See description of os_malloc() for reasoning."]
                #[doc = ""]
                #[doc = " Free's memory allocated by malloc."]
                #[doc = ""]
                #[doc = " - __`mem`__: The memory to free."]
                pub fn os_free(mem: *mut ::cty::c_void);
            }
            extern "C" {
                #[doc =
                      " Operating system level realloc(). See description of os_malloc() for reasoning."]
                #[doc = ""]
                #[doc =
                      " Reallocates the memory at ptr, to be size contiguouos bytes."]
                #[doc = ""]
                #[doc = " - __`ptr`__: A pointer to the memory to allocate"]
                #[doc =
                      " - __`size`__: The number of contiguouos bytes to allocate at that location"]
                #[doc = ""]
                #[doc =
                      " Return: A pointer to memory of size, or NULL on failure to allocate"]
                pub fn os_realloc(ptr: *mut ::cty::c_void, size: usize)
                 -> *mut ::cty::c_void;
            }
            #[doc =
                  " A mbuf pool from which to allocate mbufs. This contains a pointer to the os"]
            #[doc =
                  " mempool to allocate mbufs out of, the total number of elements in the pool,"]
            #[doc =
                  " and the amount of \"user\" data in a non-packet header mbuf. The total pool"]
            #[doc = " size, in bytes, should be:"]
            #[doc =
                  "  os_mbuf_count * (omp_databuf_len + sizeof(struct os_mbuf))"]
            #[repr(C)]
            pub struct os_mbuf_pool {
                #[doc =
                      " Total length of the databuf in each mbuf.  This is the size of the"]
                #[doc = " mempool block, minus the mbuf header"]
                pub omp_databuf_len: u16,
                #[doc = " The memory pool which to allocate mbufs out of"]
                pub omp_pool: *mut os_mempool,
                pub omp_next: os_mbuf_pool__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_mbuf_pool__bindgen_ty_1 {
                pub stqe_next: *mut os_mbuf_pool,
            }
            impl Default for os_mbuf_pool__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mbuf_pool {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc =
                  " A packet header structure that preceeds the mbuf packet headers."]
            #[repr(C)]
            pub struct os_mbuf_pkthdr {
                #[doc = " Overall length of the packet."]
                pub omp_len: u16,
                #[doc = " Flags"]
                pub omp_flags: u16,
                pub omp_next: os_mbuf_pkthdr__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_mbuf_pkthdr__bindgen_ty_1 {
                pub stqe_next: *mut os_mbuf_pkthdr,
            }
            impl Default for os_mbuf_pkthdr__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mbuf_pkthdr {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Chained memory buffer."]
            #[repr(C)]
            pub struct os_mbuf {
                #[doc = " Current pointer to data in the structure"]
                pub om_data: *mut u8,
                #[doc =
                      " Flags associated with this buffer, see OS_MBUF_F_* defintions"]
                pub om_flags: u8,
                #[doc = " Length of packet header"]
                pub om_pkthdr_len: u8,
                #[doc = " Length of data in this buffer"]
                pub om_len: u16,
                #[doc = " The mbuf pool this mbuf was allocated out of"]
                pub om_omp: *mut os_mbuf_pool,
                pub om_next: os_mbuf__bindgen_ty_1,
                #[doc =
                      " Pointer to the beginning of the data, after this buffer"]
                pub om_databuf: __IncompleteArrayField<u8>,
            }
            #[repr(C)]
            pub struct os_mbuf__bindgen_ty_1 {
                pub sle_next: *mut os_mbuf,
            }
            impl Default for os_mbuf__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mbuf {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Structure representing a queue of mbufs."]
            #[repr(C)]
            pub struct os_mqueue {
                pub mq_head: os_mqueue__bindgen_ty_1,
                #[doc =
                      " Event to post when new buffers are available on the queue."]
                pub mq_ev: os_event,
            }
            #[repr(C)]
            pub struct os_mqueue__bindgen_ty_1 {
                pub stqh_first: *mut os_mbuf_pkthdr,
                pub stqh_last: *mut *mut os_mbuf_pkthdr,
            }
            impl Default for os_mqueue__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mqueue {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc =
                      " Initializes an mqueue.  An mqueue is a queue of mbufs that ties to a"]
                #[doc =
                      " particular task's event queue.  Mqueues form a helper API around a common"]
                #[doc =
                      " paradigm: wait on an event queue until at least one packet is available,"]
                #[doc = " then process a queue of packets."]
                #[doc = ""]
                #[doc =
                      " When mbufs are available on the queue, an event OS_EVENT_T_MQUEUE_DATA"]
                #[doc = " will be posted to the task's mbuf queue."]
                #[doc = ""]
                #[doc =
                      " - __`mq`__:                    The mqueue to initialize"]
                #[doc =
                      " - __`ev_cb`__:                 The callback to associate with the mqeueue"]
                #[doc =
                      "                                  event.  Typically, this callback pulls each"]
                #[doc =
                      "                                  packet off the mqueue and processes them."]
                #[doc =
                      " - __`arg`__:                   The argument to associate with the mqueue event."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 on success, non-zero on failure."]
                pub fn os_mqueue_init(mq: *mut os_mqueue, ev_cb: os_event_fn,
                                      arg: *mut ::cty::c_void)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Remove and return a single mbuf from the mbuf queue.  Does not block."]
                #[doc = ""]
                #[doc =
                      " - __`mq`__: The mbuf queue to pull an element off of."]
                #[doc = ""]
                #[doc =
                      " Return: The next mbuf in the queue, or NULL if queue has no mbufs."]
                pub fn os_mqueue_get(arg1: *mut os_mqueue) -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Adds a packet (i.e. packet header mbuf) to an mqueue. The event associated"]
                #[doc =
                      " with the mqueue gets posted to the specified eventq."]
                #[doc = ""]
                #[doc =
                      " - __`mq`__:                    The mbuf queue to append the mbuf to."]
                #[doc =
                      " - __`evq`__:                   The event queue to post an event to."]
                #[doc =
                      " - __`m`__:                     The mbuf to append to the mbuf queue."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_mqueue_put(arg1: *mut os_mqueue,
                                     arg2: *mut os_eventq, arg3: *mut os_mbuf)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " MSYS is a system level mbuf registry.  Allows the system to share"]
                #[doc =
                      " packet buffers amongst the various networking stacks that can be running"]
                #[doc = " simultaeneously."]
                #[doc = ""]
                #[doc =
                      " Mbuf pools are created in the system initialization code, and then when"]
                #[doc =
                      " a mbuf is allocated out of msys, it will try and find the best fit based"]
                #[doc = " upon estimated mbuf size."]
                #[doc = ""]
                #[doc =
                      " os_msys_register() registers a mbuf pool with MSYS, and allows MSYS to"]
                #[doc = " allocate mbufs out of it."]
                #[doc = ""]
                #[doc = " - __`new_pool`__: The pool to register with MSYS"]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure"]
                pub fn os_msys_register(arg1: *mut os_mbuf_pool)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Allocate a mbuf from msys.  Based upon the data size requested,"]
                #[doc =
                      " os_msys_get() will choose the mbuf pool that has the best fit."]
                #[doc = ""]
                #[doc =
                      " - __`dsize`__: The estimated size of the data being stored in the mbuf"]
                #[doc =
                      " - __`leadingspace`__: The amount of leadingspace to allocate in the mbuf"]
                #[doc = ""]
                #[doc =
                      " Return: A freshly allocated mbuf on success, NULL on failure."]
                pub fn os_msys_get(dsize: u16, leadingspace: u16)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc = " De-registers all mbuf pools from msys."]
                pub fn os_msys_reset();
            }
            extern "C" {
                #[doc =
                      " Allocate a packet header structure from the MSYS pool.  See"]
                #[doc = " os_msys_register() for a description of MSYS."]
                #[doc = ""]
                #[doc =
                      " - __`dsize`__: The estimated size of the data being stored in the mbuf"]
                #[doc =
                      " - __`user_hdr_len`__: The length to allocate for the packet header structure"]
                #[doc = ""]
                #[doc =
                      " Return: A freshly allocated mbuf on success, NULL on failure."]
                pub fn os_msys_get_pkthdr(dsize: u16, user_hdr_len: u16)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Count the number of blocks in all the mbuf pools that are allocated."]
                #[doc = ""]
                #[doc = " Return: total number of blocks allocated in Msys"]
                pub fn os_msys_count() -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Return the number of free blocks in Msys"]
                #[doc = ""]
                #[doc = " Return: Number of free blocks available in Msys"]
                pub fn os_msys_num_free() -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Initialize a pool of mbufs."]
                #[doc = ""]
                #[doc = " - __`omp`__:     The mbuf pool to initialize"]
                #[doc =
                      " - __`mp`__:      The memory pool that will hold this mbuf pool"]
                #[doc = " - __`buf_len`__: The length of the buffer itself."]
                #[doc = " - __`nbufs`__:   The number of buffers in the pool"]
                #[doc = ""]
                #[doc = " Return: 0 on success, error code on failure."]
                pub fn os_mbuf_pool_init(arg1: *mut os_mbuf_pool,
                                         mp: *mut os_mempool, arg2: u16,
                                         arg3: u16) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Get an mbuf from the mbuf pool.  The mbuf is allocated, and initialized"]
                #[doc = " prior to being returned."]
                #[doc = ""]
                #[doc =
                      " - __`omp`__: The mbuf pool to return the packet from"]
                #[doc =
                      " - __`leadingspace`__: The amount of leadingspace to put before the data"]
                #[doc = "     section by default."]
                #[doc = ""]
                #[doc =
                      " Return: An initialized mbuf on success, and NULL on failure."]
                pub fn os_mbuf_get(omp: *mut os_mbuf_pool, arg1: u16)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Allocate a new packet header mbuf out of the os_mbuf_pool."]
                #[doc = ""]
                #[doc = " - __`omp`__: The mbuf pool to allocate out of"]
                #[doc =
                      " - __`user_pkthdr_len`__: The packet header length to reserve for the caller."]
                #[doc = ""]
                #[doc =
                      " Return: A freshly allocated mbuf on success, NULL on failure."]
                pub fn os_mbuf_get_pkthdr(omp: *mut os_mbuf_pool,
                                          pkthdr_len: u8) -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Duplicate a chain of mbufs.  Return the start of the duplicated chain."]
                #[doc = ""]
                #[doc = " - __`omp`__: The mbuf pool to duplicate out of"]
                #[doc = " - __`om`__:  The mbuf chain to duplicate"]
                #[doc = ""]
                #[doc = " Return: A pointer to the new chain of mbufs"]
                pub fn os_mbuf_dup(m: *mut os_mbuf) -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Locates the specified absolute offset within an mbuf chain.  The offset"]
                #[doc =
                      " can be one past than the total length of the chain, but no greater."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The start of the mbuf chain to seek within."]
                #[doc =
                      " - __`off`__:                   The absolute address to find."]
                #[doc =
                      " - __`out_off`__:               On success, this points to the relative offset"]
                #[doc =
                      "                                  within the returned mbuf."]
                #[doc = ""]
                #[doc =
                      " Return:                      The mbuf containing the specified offset on"]
                #[doc = "                                  success."]
                #[doc =
                      "                              NULL if the specified offset is out of bounds."]
                pub fn os_mbuf_off(om: *const os_mbuf, off: ::cty::c_int,
                                   out_off: *mut u16) -> *mut os_mbuf;
            }
            extern "C" {
                pub fn os_mbuf_copydata(m: *const os_mbuf, off: ::cty::c_int,
                                        len: ::cty::c_int,
                                        dst: *mut ::cty::c_void)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " @brief Calculates the length of an mbuf chain."]
                #[doc = ""]
                #[doc =
                      " Calculates the length of an mbuf chain.  If the mbuf contains a packet"]
                #[doc =
                      " header, you should use `OS_MBUF_PKTLEN()` as a more efficient alternative to"]
                #[doc = " this function."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The mbuf to measure."]
                #[doc = ""]
                #[doc =
                      " Return:                      The length, in bytes, of the provided mbuf"]
                #[doc = "                                  chain."]
                pub fn os_mbuf_len(om: *const os_mbuf) -> u16;
            }
            extern "C" {
                #[doc = " Append data onto a mbuf"]
                #[doc = ""]
                #[doc = " - __`om`__:   The mbuf to append the data onto"]
                #[doc = " - __`data`__: The data to append onto the mbuf"]
                #[doc = " - __`len`__:  The length of the data to append"]
                #[doc = ""]
                #[doc = " Return: 0 on success, and an error code on failure"]
                pub fn os_mbuf_append(m: *mut os_mbuf,
                                      arg1: *const ::cty::c_void, arg2: u16)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Reads data from one mbuf and appends it to another.  On error, the specified"]
                #[doc =
                      " data range may be partially appended.  Neither mbuf is required to contain"]
                #[doc = " an mbuf packet header."]
                #[doc = ""]
                #[doc =
                      " - __`dst`__:                   The mbuf to append to."]
                #[doc =
                      " - __`src`__:                   The mbuf to copy data from."]
                #[doc =
                      " - __`src_off`__:               The absolute offset within the source mbuf"]
                #[doc =
                      "                                  chain to read from."]
                #[doc =
                      " - __`len`__:                   The number of bytes to append."]
                #[doc = ""]
                #[doc = " Return:                      0 on success;"]
                #[doc =
                      "                              OS_EINVAL if the specified range extends beyond"]
                #[doc =
                      "                                  the end of the source mbuf chain."]
                pub fn os_mbuf_appendfrom(dst: *mut os_mbuf,
                                          src: *const os_mbuf, src_off: u16,
                                          len: u16) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Release a mbuf back to the pool"]
                #[doc = ""]
                #[doc = " - __`omp`__: The Mbuf pool to release back to"]
                #[doc = " - __`om`__:  The Mbuf to release back to the pool"]
                #[doc = ""]
                #[doc = " Return: 0 on success, -1 on failure"]
                pub fn os_mbuf_free(mb: *mut os_mbuf) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Free a chain of mbufs"]
                #[doc = ""]
                #[doc =
                      " - __`omp`__: The mbuf pool to free the chain of mbufs into"]
                #[doc =
                      " - __`om`__:  The starting mbuf of the chain to free back into the pool"]
                #[doc = ""]
                #[doc = " Return: 0 on success, -1 on failure"]
                pub fn os_mbuf_free_chain(om: *mut os_mbuf) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Adjust the length of a mbuf, trimming either from the head or the tail"]
                #[doc = " of the mbuf."]
                #[doc = ""]
                #[doc = " - __`mp`__: The mbuf chain to adjust"]
                #[doc =
                      " - __`req_len`__: The length to trim from the mbuf.  If positive, trims"]
                #[doc =
                      "                from the head of the mbuf, if negative, trims from the"]
                #[doc = "                tail of the mbuf."]
                pub fn os_mbuf_adj(mp: *mut os_mbuf, req_len: ::cty::c_int);
            }
            extern "C" {
                #[doc =
                      " Performs a memory compare of the specified region of an mbuf chain against a"]
                #[doc = " flat buffer."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The start of the mbuf chain to compare."]
                #[doc =
                      " - __`off`__:                   The offset within the mbuf chain to start the"]
                #[doc = "                                  comparison."]
                #[doc =
                      " - __`data`__:                  The flat buffer to compare."]
                #[doc =
                      " - __`len`__:                   The length of the flat buffer."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 if both memory regions are identical;"]
                #[doc =
                      "                              A memcmp return code if there is a mismatch;"]
                #[doc =
                      "                              INT_MAX if the mbuf is too short."]
                pub fn os_mbuf_cmpf(om: *const os_mbuf, off: ::cty::c_int,
                                    data: *const ::cty::c_void,
                                    len: ::cty::c_int) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Compares the contents of two mbuf chains.  The ranges of the two chains to"]
                #[doc =
                      " be compared are specified via the two offset parameters and the len"]
                #[doc =
                      " parameter.  Neither mbuf chain is required to contain a packet header."]
                #[doc = ""]
                #[doc =
                      " - __`om1`__:                   The first mbuf chain to compare."]
                #[doc =
                      " - __`offset1`__:               The absolute offset within om1 at which to"]
                #[doc =
                      "                                  start the comparison."]
                #[doc =
                      " - __`om2`__:                   The second mbuf chain to compare."]
                #[doc =
                      " - __`offset2`__:               The absolute offset within om2 at which to"]
                #[doc =
                      "                                  start the comparison."]
                #[doc =
                      " - __`len`__:                   The number of bytes to compare."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 if both mbuf segments are identical;"]
                #[doc =
                      "                              A memcmp() return code if the segment contents"]
                #[doc = "                                  differ;"]
                #[doc =
                      "                              INT_MAX if a specified range extends beyond the"]
                #[doc =
                      "                                  end of its corresponding mbuf chain."]
                pub fn os_mbuf_cmpm(om1: *const os_mbuf, offset1: u16,
                                    om2: *const os_mbuf, offset2: u16,
                                    len: u16) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Increases the length of an mbuf chain by adding data to the front.  If there"]
                #[doc =
                      " is insufficient room in the leading mbuf, additional mbufs are allocated and"]
                #[doc =
                      " prepended as necessary.  If this function fails to allocate an mbuf, the"]
                #[doc = " entire chain is freed."]
                #[doc = ""]
                #[doc =
                      " The specified mbuf chain does not need to contain a packet header."]
                #[doc = ""]
                #[doc =
                      " - __`omp`__:                   The mbuf pool to allocate from."]
                #[doc =
                      " - __`om`__:                    The head of the mbuf chain."]
                #[doc =
                      " - __`len`__:                   The number of bytes to prepend."]
                #[doc = ""]
                #[doc =
                      " Return:                      The new head of the chain on success;"]
                #[doc = "                              NULL on failure."]
                pub fn os_mbuf_prepend(om: *mut os_mbuf, len: ::cty::c_int)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Prepends a chunk of empty data to the specified mbuf chain and ensures the"]
                #[doc =
                      " chunk is contiguous.  If either operation fails, the specified mbuf chain is"]
                #[doc = " freed and NULL is returned."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The mbuf chain to prepend to."]
                #[doc =
                      " - __`len`__:                   The number of bytes to prepend and pullup."]
                #[doc = ""]
                #[doc =
                      " Return:                      The modified mbuf on success;"]
                #[doc =
                      "                              NULL on failure (and the mbuf chain is freed)."]
                pub fn os_mbuf_prepend_pullup(om: *mut os_mbuf, len: u16)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Copies the contents of a flat buffer into an mbuf chain, starting at the"]
                #[doc =
                      " specified destination offset.  If the mbuf is too small for the source data,"]
                #[doc =
                      " it is extended as necessary.  If the destination mbuf contains a packet"]
                #[doc = " header, the header length is updated."]
                #[doc = ""]
                #[doc =
                      " - __`omp`__:                   The mbuf pool to allocate from."]
                #[doc =
                      " - __`om`__:                    The mbuf chain to copy into."]
                #[doc =
                      " - __`off`__:                   The offset within the chain to copy to."]
                #[doc =
                      " - __`src`__:                   The source buffer to copy from."]
                #[doc =
                      " - __`len`__:                   The number of bytes to copy."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 on success; nonzero on failure."]
                pub fn os_mbuf_copyinto(om: *mut os_mbuf, off: ::cty::c_int,
                                        src: *const ::cty::c_void,
                                        len: ::cty::c_int) -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Attaches a second mbuf chain onto the end of the first.  If the first chain"]
                #[doc =
                      " contains a packet header, the header's length is updated.  If the second"]
                #[doc = " chain has a packet header, its header is cleared."]
                #[doc = ""]
                #[doc =
                      " - __`first`__:                 The mbuf chain being attached to."]
                #[doc =
                      " - __`second`__:                The mbuf chain that gets attached."]
                pub fn os_mbuf_concat(first: *mut os_mbuf,
                                      second: *mut os_mbuf);
            }
            extern "C" {
                #[doc =
                      " Increases the length of an mbuf chain by the specified amount.  If there is"]
                #[doc =
                      " not sufficient room in the last buffer, a new buffer is allocated and"]
                #[doc =
                      " appended to the chain.  It is an error to request more data than can fit in"]
                #[doc = " a single buffer."]
                #[doc = ""]
                #[doc = " @param omp"]
                #[doc =
                      " - __`om`__:                    The head of the chain to extend."]
                #[doc =
                      " - __`len`__:                   The number of bytes to extend by."]
                #[doc = ""]
                #[doc =
                      " Return:                      A pointer to the new data on success;"]
                #[doc = "                              NULL on failure."]
                pub fn os_mbuf_extend(om: *mut os_mbuf, len: u16)
                 -> *mut ::cty::c_void;
            }
            extern "C" {
                #[doc =
                      " Rearrange a mbuf chain so that len bytes are contiguous,"]
                #[doc =
                      " and in the data area of an mbuf (so that OS_MBUF_DATA() will"]
                #[doc =
                      " work on a structure of size len.)  Returns the resulting"]
                #[doc =
                      " mbuf chain on success, free's it and returns NULL on failure."]
                #[doc = ""]
                #[doc =
                      " If there is room, it will add up to \"max_protohdr - len\""]
                #[doc =
                      " extra bytes to the contiguous region, in an attempt to avoid being"]
                #[doc = " called next time."]
                #[doc = ""]
                #[doc =
                      " - __`omp`__: The mbuf pool to take the mbufs out of"]
                #[doc = " - __`om`__: The mbuf chain to make contiguous"]
                #[doc =
                      " - __`len`__: The number of bytes in the chain to make contiguous"]
                #[doc = ""]
                #[doc =
                      " Return: The contiguous mbuf chain on success, NULL on failure."]
                pub fn os_mbuf_pullup(om: *mut os_mbuf, len: u16)
                 -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Removes and frees empty mbufs from the front of a chain.  If the chain"]
                #[doc = " contains a packet header, it is preserved."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The mbuf chain to trim."]
                #[doc = ""]
                #[doc =
                      " Return:                      The head of the trimmed mbuf chain."]
                pub fn os_mbuf_trim_front(om: *mut os_mbuf) -> *mut os_mbuf;
            }
            extern "C" {
                #[doc =
                      " Increases the length of an mbuf chain by inserting a gap at the specified"]
                #[doc =
                      " offset.  The contents of the gap are indeterminate.  If the mbuf chain"]
                #[doc =
                      " contains a packet header, its total length is increased accordingly."]
                #[doc = ""]
                #[doc = " This function never frees the provided mbuf chain."]
                #[doc = ""]
                #[doc =
                      " - __`om`__:                    The mbuf chain to widen."]
                #[doc =
                      " - __`off`__:                   The offset at which to insert the gap."]
                #[doc =
                      " - __`len`__:                   The size of the gap to insert."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 on success; SYS_[...] error code on failure."]
                pub fn os_mbuf_widen(om: *mut os_mbuf, off: u16, len: u16)
                 -> ::cty::c_int;
            }
            #[doc =
                  " A memory block structure. This simply contains a pointer to the free list"]
            #[doc =
                  " chain and is only used when the block is on the free list. When the block"]
            #[doc =
                  " has been removed from the free list the entire memory block is usable by the"]
            #[doc = " caller."]
            #[repr(C)]
            pub struct os_memblock {
                pub mb_next: os_memblock__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_memblock__bindgen_ty_1 {
                pub sle_next: *mut os_memblock,
            }
            impl Default for os_memblock__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_memblock {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Memory pool"]
            #[repr(C)]
            pub struct os_mempool {
                #[doc = " Size of the memory blocks, in bytes."]
                pub mp_block_size: u32,
                #[doc = " The number of memory blocks."]
                pub mp_num_blocks: u16,
                #[doc = " The number of free blocks left"]
                pub mp_num_free: u16,
                #[doc = " The lowest number of free blocks seen"]
                pub mp_min_free: u16,
                #[doc = " Bitmap of OS_MEMPOOL_F_[...] values."]
                pub mp_flags: u8,
                #[doc = " Address of memory buffer used by pool"]
                pub mp_membuf_addr: u32,
                pub mp_list: os_mempool__bindgen_ty_1,
                pub __bindgen_anon_1: os_mempool__bindgen_ty_2,
                #[doc = " Name for memory block"]
                pub name: *mut ::cty::c_char,
            }
            #[repr(C)]
            pub struct os_mempool__bindgen_ty_1 {
                pub stqe_next: *mut os_mempool,
            }
            impl Default for os_mempool__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_mempool__bindgen_ty_2 {
                pub slh_first: *mut os_memblock,
            }
            impl Default for os_mempool__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mempool {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc =
                  " Block put callback function.  If configured, this callback gets executed"]
            #[doc =
                  " whenever a block is freed to the corresponding extended mempool.  Note: The"]
            #[doc =
                  " os_memblock_put() function calls this callback instead of freeing the block"]
            #[doc =
                  " itself.  Therefore, it is the callback's responsibility to free the block"]
            #[doc = " via a call to os_memblock_put_from_cb()."]
            #[doc = ""]
            #[doc =
                  " - __`ome`__:                   The extended mempool that a block is being"]
            #[doc = "                                  freed back to."]
            #[doc = " - __`data`__:                  The block being freed."]
            #[doc =
                  " - __`arg`__:                   Optional argument configured along with the"]
            #[doc = "                                  callback."]
            #[doc = ""]
            #[doc =
                  " Return:                      Indicates whether the block was successfully"]
            #[doc =
                  "                                  freed.  A non-zero value should only be"]
            #[doc =
                  "                                  returned if the block was not successfully"]
            #[doc =
                  "                                  released back to its pool."]
            pub type os_mempool_put_fn
                =
                ::core::option::Option<unsafe extern "C" fn(ome:
                                                                *mut os_mempool_ext,
                                                            data:
                                                                *mut ::cty::c_void,
                                                            arg:
                                                                *mut ::cty::c_void)
                                           -> os_error_t>;
            #[repr(C)]
            pub struct os_mempool_ext {
                pub mpe_mp: os_mempool,
                pub mpe_put_cb: os_mempool_put_fn,
                pub mpe_put_arg: *mut ::cty::c_void,
            }
            impl Default for os_mempool_ext {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc =
                  " Information describing a memory pool, used to return OS information"]
            #[doc = " to the management layer."]
            #[repr(C)]
            pub struct os_mempool_info {
                #[doc = " Size of the memory blocks in the pool"]
                pub omi_block_size: ::cty::c_int,
                #[doc = " Number of memory blocks in the pool"]
                pub omi_num_blocks: ::cty::c_int,
                #[doc = " Number of free memory blocks"]
                pub omi_num_free: ::cty::c_int,
                #[doc = " Minimum number of free memory blocks ever"]
                pub omi_min_free: ::cty::c_int,
                #[doc = " Name of the memory pool"]
                pub omi_name: [::cty::c_char; 32usize],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for os_mempool_info {
                #[inline]
                fn default() -> os_mempool_info {
                    os_mempool_info{omi_block_size:
                                        ::core::default::Default::default(),
                                    omi_num_blocks:
                                        ::core::default::Default::default(),
                                    omi_num_free:
                                        ::core::default::Default::default(),
                                    omi_min_free:
                                        ::core::default::Default::default(),
                                    omi_name:
                                        ::core::default::Default::default(),}
                }
            }
            extern "C" {
                #[doc = " Get information about the next system memory pool."]
                #[doc = ""]
                #[doc =
                      " - __`mempool`__: The current memory pool, or NULL if starting iteration."]
                #[doc =
                      " - __`info`__:    A pointer to the structure to return memory pool information"]
                #[doc = "                into."]
                #[doc = ""]
                #[doc =
                      " Return: The next memory pool in the list to get information about, or NULL"]
                #[doc = "         when at the last memory pool."]
                pub fn os_mempool_info_get_next(arg1: *mut os_mempool,
                                                arg2: *mut os_mempool_info)
                 -> *mut os_mempool;
            }
            pub type os_membuf_t = u32;
            extern "C" {
                #[doc = " Initialize a memory pool."]
                #[doc = ""]
                #[doc =
                      " - __`mp`__:            Pointer to a pointer to a mempool"]
                #[doc =
                      " - __`blocks`__:        The number of blocks in the pool"]
                #[doc =
                      " - __`blocks_size`__:   The size of the block, in bytes."]
                #[doc =
                      " - __`membuf`__:        Pointer to memory to contain blocks."]
                #[doc = " - __`name`__:          Name of the pool."]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                pub fn os_mempool_init(mp: *mut os_mempool, blocks: u16,
                                       block_size: u32,
                                       membuf: *mut ::cty::c_void,
                                       name: *mut ::cty::c_char)
                 -> os_error_t;
            }
            extern "C" {
                #[doc =
                      " Initializes an extended memory pool.  Extended attributes (e.g., callbacks)"]
                #[doc =
                      " are not specified when this function is called; they are assigned manually"]
                #[doc = " after initialization."]
                #[doc = ""]
                #[doc =
                      " - __`mpe`__:           The extended memory pool to initialize."]
                #[doc =
                      " - __`blocks`__:        The number of blocks in the pool."]
                #[doc =
                      " - __`block_size`__:    The size of each block, in bytes."]
                #[doc =
                      " - __`membuf`__:        Pointer to memory to contain blocks."]
                #[doc = " - __`name`__:          Name of the pool."]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                pub fn os_mempool_ext_init(mpe: *mut os_mempool_ext,
                                           blocks: u16, block_size: u32,
                                           membuf: *mut ::cty::c_void,
                                           name: *mut ::cty::c_char)
                 -> os_error_t;
            }
            extern "C" {
                #[doc =
                      " Removes the specified mempool from the list of initialized mempools."]
                #[doc = ""]
                #[doc =
                      " - __`mp`__:                    The mempool to unregister."]
                #[doc = ""]
                #[doc = " Return:                      0 on success;"]
                #[doc =
                      "                              OS_INVALID_PARM if the mempool is not"]
                #[doc = "                                  registered."]
                pub fn os_mempool_unregister(mp: *mut os_mempool)
                 -> os_error_t;
            }
            extern "C" {
                #[doc = " Clears a memory pool."]
                #[doc = ""]
                #[doc = " - __`mp`__:            The mempool to clear."]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                pub fn os_mempool_clear(mp: *mut os_mempool) -> os_error_t;
            }
            extern "C" {
                pub fn os_mempool_is_sane(mp: *const os_mempool) -> bool;
            }
            extern "C" {
                #[doc =
                      " Checks if a memory block was allocated from the specified mempool."]
                #[doc = ""]
                #[doc =
                      " - __`mp`__:                    The mempool to check as parent."]
                #[doc =
                      " - __`block_addr`__:            The memory block to check as child."]
                #[doc = ""]
                #[doc =
                      " Return:                      0 if the block does not belong to the mempool;"]
                #[doc =
                      "                              1 if the block does belong to the mempool."]
                pub fn os_memblock_from(mp: *const os_mempool,
                                        block_addr: *const ::cty::c_void)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Get a memory block from a memory pool"]
                #[doc = ""]
                #[doc = " - __`mp`__: Pointer to the memory pool"]
                #[doc = ""]
                #[doc =
                      " Return: void* Pointer to block if available; NULL otherwise"]
                pub fn os_memblock_get(mp: *mut os_mempool)
                 -> *mut ::cty::c_void;
            }
            extern "C" {
                #[doc =
                      " Puts the memory block back into the pool, ignoring the put callback, if any."]
                #[doc =
                      " This function should only be called from a put callback to free a block"]
                #[doc = " without causing infinite recursion."]
                #[doc = ""]
                #[doc = " - __`mp`__: Pointer to memory pool"]
                #[doc = " - __`block_addr`__: Pointer to memory block"]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                pub fn os_memblock_put_from_cb(mp: *mut os_mempool,
                                               block_addr: *mut ::cty::c_void)
                 -> os_error_t;
            }
            extern "C" {
                #[doc = " Puts the memory block back into the pool"]
                #[doc = ""]
                #[doc = " - __`mp`__: Pointer to memory pool"]
                #[doc = " - __`block_addr`__: Pointer to memory block"]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                pub fn os_memblock_put(mp: *mut os_mempool,
                                       block_addr: *mut ::cty::c_void)
                 -> os_error_t;
            }
            #[doc = " OS mutex structure"]
            #[repr(C)]
            pub struct os_mutex {
                pub mu_head: os_mutex__bindgen_ty_1,
                pub _pad: u8,
                #[doc = " Mutex owner's default priority"]
                pub mu_prio: u8,
                #[doc = " Mutex call nesting level"]
                pub mu_level: u16,
                #[doc = " Task that owns the mutex"]
                pub mu_owner: *mut os_task,
            }
            #[repr(C)]
            pub struct os_mutex__bindgen_ty_1 {
                pub slh_first: *mut os_task,
            }
            impl Default for os_mutex__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mutex {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Create a mutex and initialize it."]
                #[doc = ""]
                #[doc = " - __`mu`__: Pointer to mutex"]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc = "      OS_INVALID_PARM     Mutex passed in was NULL."]
                #[doc = "      OS_OK               no error."]
                pub fn os_mutex_init(mu: *mut os_mutex) -> os_error_t;
            }
            extern "C" {
                #[doc = " Release a mutex."]
                #[doc = ""]
                #[doc = " - __`mu`__: Pointer to the mutex to be released"]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc = "      OS_INVALID_PARM Mutex passed in was NULL."]
                #[doc =
                      "      OS_BAD_MUTEX    Mutex was not granted to current task (not owner)."]
                #[doc = "      OS_OK           No error"]
                pub fn os_mutex_release(mu: *mut os_mutex) -> os_error_t;
            }
            extern "C" {
                #[doc = " Pend (wait) for a mutex."]
                #[doc = ""]
                #[doc = " - __`mu`__: Pointer to mutex."]
                #[doc = " - __`timeout`__: Timeout, in os ticks."]
                #[doc =
                      "                A timeout of 0 means do not wait if not available."]
                #[doc =
                      "                A timeout of OS_TIMEOUT_NEVER means wait forever."]
                #[doc = ""]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc = "      OS_INVALID_PARM     Mutex passed in was NULL."]
                #[doc =
                      "      OS_TIMEOUT          Mutex was owned by another task and timeout=0"]
                #[doc = "      OS_OK               no error."]
                pub fn os_mutex_pend(mu: *mut os_mutex, timeout: os_time_t)
                 -> os_error_t;
            }
            pub type os_sanity_check_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut os_sanity_check,
                                                            arg2:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct os_sanity_check {
                #[doc = " Time this check last ran successfully."]
                pub sc_checkin_last: os_time_t,
                #[doc = " Interval this task should check in at"]
                pub sc_checkin_itvl: os_time_t,
                #[doc = " Sanity check to run"]
                pub sc_func: os_sanity_check_func_t,
                #[doc = " Argument to pass to sanity check"]
                pub sc_arg: *mut ::cty::c_void,
                pub sc_next: os_sanity_check__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_sanity_check__bindgen_ty_1 {
                pub sle_next: *mut os_sanity_check,
            }
            impl Default for os_sanity_check__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_sanity_check {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " @cond INTERNAL_HIDDEN"]
                pub fn os_sanity_init() -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_sanity_run();
            }
            extern "C" {
                #[doc = " Provide a \"task checkin\" for the sanity task."]
                #[doc = ""]
                #[doc = " - __`t`__: The task to check in"]
                #[doc = ""]
                #[doc = " Return: 0 on success, error code on failure"]
                pub fn os_sanity_task_checkin(arg1: *mut os_task)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Initialize a sanity check"]
                #[doc = ""]
                #[doc = " - __`sc`__: The sanity check to initialize"]
                #[doc = ""]
                #[doc = " Return: 0 on success, error code on failure."]
                pub fn os_sanity_check_init(arg1: *mut os_sanity_check)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Register a sanity check"]
                #[doc = ""]
                #[doc = " - __`sc`__: The sanity check to register"]
                #[doc = ""]
                #[doc = " Return: 0 on success, error code on failure"]
                pub fn os_sanity_check_register(arg1: *mut os_sanity_check)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Reset the os sanity check, so that it doesn't trip up the"]
                #[doc = " sanity timer."]
                #[doc = ""]
                #[doc = " - __`sc`__: The sanity check to reset"]
                #[doc = ""]
                #[doc = " Return: 0 on success, error code on failure"]
                pub fn os_sanity_check_reset(arg1: *mut os_sanity_check)
                 -> ::cty::c_int;
            }
            #[repr(C)]
            pub struct os_task_obj {
                pub obj_head: os_task_obj__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_task_obj__bindgen_ty_1 {
                pub slh_first: *mut os_task,
            }
            impl Default for os_task_obj__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_task_obj {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Task is ready to run"]
            pub const os_task_state_OS_TASK_READY: os_task_state = 1;
            #[doc = " Task is sleeping"]
            pub const os_task_state_OS_TASK_SLEEP: os_task_state = 2;
            #[doc = " Task states"]
            pub type os_task_state = u32;
            pub use self::os_task_state as os_task_state_t;
            pub type os_task_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut ::cty::c_void)>;
            #[doc = " @cond INTERNAL_HIDDEN"]
            #[repr(C)]
            pub struct os_task {
                #[doc = " Current stack pointer for this task"]
                pub t_stackptr: *mut os_stack_t,
                #[doc = " Pointer to top of this task's stack"]
                pub t_stacktop: *mut os_stack_t,
                #[doc = " Size of this task's stack"]
                pub t_stacksize: u16,
                #[doc = " Task ID"]
                pub t_taskid: u8,
                #[doc = " Task Priority"]
                pub t_prio: u8,
                pub t_state: u8,
                #[doc = " Task flags, bitmask"]
                pub t_flags: u8,
                pub t_lockcnt: u8,
                pub t_pad: u8,
                #[doc = " Task name"]
                pub t_name: *const ::cty::c_char,
                #[doc = " Task function that executes"]
                pub t_func: os_task_func_t,
                #[doc = " Argument to pass to task function when called"]
                pub t_arg: *mut ::cty::c_void,
                #[doc =
                      " Current object task is waiting on, either a semaphore or mutex"]
                pub t_obj: *mut ::cty::c_void,
                #[doc = " Default sanity check for this task"]
                pub t_sanity_check: os_sanity_check,
                #[doc = " Next scheduled wakeup if this task is sleeping"]
                pub t_next_wakeup: os_time_t,
                #[doc = " Total task run time"]
                pub t_run_time: os_time_t,
                #[doc =
                      " Total number of times this task has been context switched during"]
                #[doc = " execution."]
                pub t_ctx_sw_cnt: u32,
                pub t_os_task_list: os_task__bindgen_ty_1,
                pub t_os_list: os_task__bindgen_ty_2,
                pub t_obj_list: os_task__bindgen_ty_3,
            }
            #[repr(C)]
            pub struct os_task__bindgen_ty_1 {
                pub stqe_next: *mut os_task,
            }
            impl Default for os_task__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_task__bindgen_ty_2 {
                pub tqe_next: *mut os_task,
                pub tqe_prev: *mut *mut os_task,
            }
            impl Default for os_task__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_task__bindgen_ty_3 {
                pub sle_next: *mut os_task,
            }
            impl Default for os_task__bindgen_ty_3 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_task {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " @cond INTERNAL_HIDDEN"]
            #[repr(C)]
            pub struct os_task_stailq {
                pub stqh_first: *mut os_task,
                pub stqh_last: *mut *mut os_task,
            }
            impl Default for os_task_stailq {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Initialize a task."]
                #[doc = ""]
                #[doc =
                      " This function initializes the task structure pointed to by t,"]
                #[doc =
                      " clearing and setting it's stack pointer, provides sane defaults"]
                #[doc =
                      " and sets the task as ready to run, and inserts it into the operating"]
                #[doc = " system scheduler."]
                #[doc = ""]
                #[doc = " - __`t`__: The task to initialize"]
                #[doc = " - __`name`__: The name of the task to initialize"]
                #[doc = " - __`func`__: The task function to call"]
                #[doc =
                      " - __`arg`__: The argument to pass to this task function"]
                #[doc =
                      " - __`prio`__: The priority at which to run this task"]
                #[doc =
                      " - __`sanity_itvl`__: The time at which this task should check in with the"]
                #[doc =
                      "                    sanity task.  OS_WAIT_FOREVER means never check in"]
                #[doc = "                    here."]
                #[doc =
                      " - __`stack_bottom`__: A pointer to the bottom of a task's stack"]
                #[doc =
                      " - __`stack_size`__: The overall size of the task's stack."]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure."]
                pub fn os_task_init(arg1: *mut os_task,
                                    arg2: *const ::cty::c_char,
                                    arg3: os_task_func_t,
                                    arg4: *mut ::cty::c_void, arg5: u8,
                                    arg6: os_time_t, arg7: *mut os_stack_t,
                                    arg8: u16) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Removes specified task"]
                #[doc = " XXX"]
                #[doc =
                      " NOTE: This interface is currently experimental and not ready for common use"]
                pub fn os_task_remove(t: *mut os_task) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = " Return the number of tasks initialized."]
                #[doc = ""]
                #[doc = " Return: number of tasks initialized"]
                pub fn os_task_count() -> u8;
            }
            #[doc =
                  " Information about an individual task, returned for management APIs."]
            #[repr(C)]
            pub struct os_task_info {
                #[doc = " Task priority"]
                pub oti_prio: u8,
                #[doc = " Task identifier"]
                pub oti_taskid: u8,
                #[doc = " Task state, either READY or SLEEP"]
                pub oti_state: u8,
                #[doc = " Task stack usage"]
                pub oti_stkusage: u16,
                #[doc = " Task stack size"]
                pub oti_stksize: u16,
                #[doc = " Task context switch count"]
                pub oti_cswcnt: u32,
                #[doc = " Task runtime"]
                pub oti_runtime: u32,
                #[doc = " Last time this task checked in with sanity"]
                pub oti_last_checkin: os_time_t,
                #[doc =
                      " Next time this task is scheduled to check-in with sanity"]
                pub oti_next_checkin: os_time_t,
                #[doc = " Name of this task"]
                pub oti_name: [::cty::c_char; 32usize],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for os_task_info {
                #[inline]
                fn default() -> os_task_info {
                    os_task_info{oti_prio:
                                     ::core::default::Default::default(),
                                 oti_taskid:
                                     ::core::default::Default::default(),
                                 oti_state:
                                     ::core::default::Default::default(),
                                 oti_stkusage:
                                     ::core::default::Default::default(),
                                 oti_stksize:
                                     ::core::default::Default::default(),
                                 oti_cswcnt:
                                     ::core::default::Default::default(),
                                 oti_runtime:
                                     ::core::default::Default::default(),
                                 oti_last_checkin:
                                     ::core::default::Default::default(),
                                 oti_next_checkin:
                                     ::core::default::Default::default(),
                                 oti_name:
                                     ::core::default::Default::default(),}
                }
            }
            extern "C" {
                #[doc =
                      " Iterate through tasks, and return the following information about them:"]
                #[doc = ""]
                #[doc = " - Priority"]
                #[doc = " - Task ID"]
                #[doc = " - State (READY, SLEEP)"]
                #[doc = " - Total Stack Usage"]
                #[doc = " - Stack Size"]
                #[doc = " - Context Switch Count"]
                #[doc = " - Runtime"]
                #[doc = " - Last & Next Sanity checkin"]
                #[doc = " - Task Name"]
                #[doc = ""]
                #[doc =
                      " To get the first task in the list, call os_task_info_get_next() with a"]
                #[doc =
                      " NULL pointer in the prev argument, and os_task_info_get_next() will"]
                #[doc =
                      " return a pointer to the task structure, and fill out the os_task_info"]
                #[doc = " structure pointed to by oti."]
                #[doc = ""]
                #[doc =
                      " To get the next task in the list, provide the task structure returned"]
                #[doc =
                      " by the previous call to os_task_info_get_next(), and os_task_info_get_next()"]
                #[doc =
                      " will fill out the task structure pointed to by oti again, and return"]
                #[doc = " the next task in the list."]
                #[doc = ""]
                #[doc =
                      " - __`prev`__: The previous task returned by os_task_info_get_next(), or NULL"]
                #[doc = "             to begin iteration."]
                #[doc =
                      " - __`oti`__:  The OS task info structure to fill out."]
                #[doc = ""]
                #[doc =
                      " Return: A pointer to the OS task that has been read, or NULL when finished"]
                #[doc = "         iterating through all tasks."]
                pub fn os_task_info_get_next(arg1: *const os_task,
                                             arg2: *mut os_task_info)
                 -> *mut os_task;
            }
            #[repr(C)]
            pub struct os_task_list {
                pub tqh_first: *mut os_task,
                pub tqh_last: *mut *mut os_task,
            }
            impl Default for os_task_list {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                pub fn os_sched_ctx_sw_hook(arg1: *mut os_task);
            }
            extern "C" {
                #[doc =
                      " Returns the currently running task. Note that this task may or may not be"]
                #[doc = " the highest priority task ready to run."]
                #[doc = ""]
                #[doc = " Return: The currently running task."]
                pub fn os_sched_get_current_task() -> *mut os_task;
            }
            extern "C" {
                pub fn os_sched_set_current_task(arg1: *mut os_task);
            }
            extern "C" {
                pub fn os_sched_next_task() -> *mut os_task;
            }
            extern "C" {
                #[doc =
                      " Performs context switch if needed. If next_t is set, that task will be made"]
                #[doc =
                      " running. If next_t is NULL, highest priority ready to run is swapped in. This"]
                #[doc =
                      " function can be called when new tasks were made ready to run or if the current"]
                #[doc = " task is moved to sleeping state."]
                #[doc = ""]
                #[doc =
                      " This function will call the architecture specific routine to swap in the new task."]
                #[doc = ""]
                #[doc =
                      " - __`next_t`__: Pointer to task which must run next (optional)"]
                #[doc = ""]
                #[doc = " Return: n/a"]
                #[doc = ""]
                #[doc =
                      " __Note:__ Interrupts must be disabled when calling this."]
                #[doc = ""]
                #[doc = " ```c"]
                #[doc = " // example"]
                #[doc = " os_error_t"]
                #[doc = " os_mutex_release(struct os_mutex *mu)"]
                #[doc = " {"]
                #[doc = "     ..."]
                #[doc = "     OS_EXIT_CRITICAL(sr);"]
                #[doc = ""]
                #[doc = "     // Re-schedule if needed"]
                #[doc = "     if (resched) {"]
                #[doc = "         os_sched(rdy);"]
                #[doc = "     }"]
                #[doc = ""]
                #[doc = "     return OS_OK;"]
                #[doc = ""]
                #[doc = " }"]
                #[doc = " ```"]
                pub fn os_sched(arg1: *mut os_task);
            }
            extern "C" {
                #[doc = " @cond INTERNAL_HIDDEN"]
                pub fn os_sched_os_timer_exp();
            }
            extern "C" {
                pub fn os_sched_insert(arg1: *mut os_task) -> os_error_t;
            }
            extern "C" {
                pub fn os_sched_sleep(arg1: *mut os_task, nticks: os_time_t)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_sched_wakeup(arg1: *mut os_task) -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_sched_remove(arg1: *mut os_task) -> ::cty::c_int;
            }
            extern "C" {
                pub fn os_sched_resort(arg1: *mut os_task);
            }
            extern "C" {
                pub fn os_sched_wakeup_ticks(now: os_time_t) -> os_time_t;
            }
            #[doc = " Structure representing an OS semaphore."]
            #[repr(C)]
            pub struct os_sem {
                pub sem_head: os_sem__bindgen_ty_1,
                pub _pad: u16,
                #[doc = " Number of tokens"]
                pub sem_tokens: u16,
            }
            #[repr(C)]
            pub struct os_sem__bindgen_ty_1 {
                pub slh_first: *mut os_task,
            }
            impl Default for os_sem__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_sem {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = " Initialize a semaphore"]
                #[doc = ""]
                #[doc = " - __`sem`__: Pointer to semaphore"]
                #[doc =
                      "        tokens: # of tokens the semaphore should contain initially."]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc =
                      "      OS_INVALID_PARM     Semaphore passed in was NULL."]
                #[doc = "      OS_OK               no error."]
                pub fn os_sem_init(sem: *mut os_sem, tokens: u16)
                 -> os_error_t;
            }
            extern "C" {
                #[doc = " Release a semaphore."]
                #[doc = ""]
                #[doc =
                      " - __`sem`__: Pointer to the semaphore to be released"]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc = "      OS_INVALID_PARM Semaphore passed in was NULL."]
                #[doc = "      OS_OK No error"]
                pub fn os_sem_release(sem: *mut os_sem) -> os_error_t;
            }
            extern "C" {
                #[doc = " os sem pend"]
                #[doc = ""]
                #[doc = " Pend (wait) for a semaphore."]
                #[doc = ""]
                #[doc = " - __`mu`__: Pointer to semaphore."]
                #[doc = " - __`timeout`__: Timeout, in os ticks."]
                #[doc =
                      "                A timeout of 0 means do not wait if not available."]
                #[doc =
                      "                A timeout of OS_TIMEOUT_NEVER means wait forever."]
                #[doc = ""]
                #[doc = ""]
                #[doc = " Return: os_error_t"]
                #[doc =
                      "      OS_INVALID_PARM     Semaphore passed in was NULL."]
                #[doc =
                      "      OS_TIMEOUT          Semaphore was owned by another task and timeout=0"]
                #[doc = "      OS_OK               no error."]
                pub fn os_sem_pend(sem: *mut os_sem, timeout: os_time_t)
                 -> os_error_t;
            }
            extern "C" {
                pub fn os_mempool_module_init();
            }
            extern "C" {
                pub fn os_msys_init();
            }
            extern "C" {
                #[doc =
                      " Set up the periodic timer to interrupt at a frequency of 'os_ticks_per_sec'."]
                #[doc =
                      " 'prio' is the cpu-specific priority of the periodic timer interrupt."]
                #[doc = ""]
                #[doc =
                      " - __`os_ticks_per_sec`__: Frequency of the OS tick timer"]
                #[doc =
                      " - __`prio`__:             Priority of the OS tick timer"]
                pub fn os_tick_init(os_ticks_per_sec: u32,
                                    prio: ::cty::c_int);
            }
            extern "C" {
                #[doc = " Halt CPU for up to 'n' ticks."]
                #[doc = ""]
                #[doc = " - __`n`__: The number of ticks to halt the CPU for"]
                pub fn os_tick_idle(n: os_time_t);
            }
            extern "C" {
                pub static mut os_main_task: os_task;
            }
            extern "C" {
                pub static mut os_main_stack: [os_stack_t; 1024usize];
            }
            extern "C" {
                #[doc =
                      " Idle operating system task, runs when no other tasks are running."]
                #[doc =
                      " The idle task operates in tickless mode, which means it looks for"]
                #[doc =
                      " the next time an event in the system needs to run, and then tells"]
                #[doc =
                      " the architecture specific functions to sleep until that time."]
                #[doc = ""]
                #[doc = " - __`arg`__: unused"]
                pub fn os_idle_task(arg: *mut ::cty::c_void);
            }
            extern "C" {
                pub fn os_pkg_init();
            }
        }
    }
    pub mod hw {
        //! Mynewt Hardware API for Rust
        pub mod sensor {
            //! Contains the Mynewt Sensor API for Rust, including the safe version of the API.
            //! Auto-generated Rust bindings are in the `bindings` module.
            use ::cty::c_void;
            use super::super::result::*;
            /// Contains the auto-generated Rust bindings for the Mynewt Sensor API
            mod bindings {
                use super::super::super::kernel::os::*;
                #[repr(C)]
                #[structural_match]
                #[rustc_copy_clone_marker]
                pub struct __BindgenBitfieldUnit<Storage, Align> where
                           Storage: AsRef<[u8]> + AsMut<[u8]> {
                    storage: Storage,
                    align: [Align; 0],
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::marker::Copy,
                      Align: ::core::marker::Copy> ::core::marker::Copy for
                 __BindgenBitfieldUnit<Storage, Align> where
                 Storage: AsRef<[u8]> + AsMut<[u8]> {
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::clone::Clone,
                      Align: ::core::clone::Clone> ::core::clone::Clone for
                 __BindgenBitfieldUnit<Storage, Align> where
                 Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    fn clone(&self) -> __BindgenBitfieldUnit<Storage, Align> {
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            __BindgenBitfieldUnit{storage:
                                                      ::core::clone::Clone::clone(&(*__self_0_0)),
                                                  align:
                                                      ::core::clone::Clone::clone(&(*__self_0_1)),},
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::fmt::Debug, Align: ::core::fmt::Debug>
                 ::core::fmt::Debug for __BindgenBitfieldUnit<Storage, Align>
                 where Storage: AsRef<[u8]> + AsMut<[u8]> {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter)
                     -> ::core::fmt::Result {
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            => {
                                let mut debug_trait_builder =
                                    f.debug_struct("__BindgenBitfieldUnit");
                                let _ =
                                    debug_trait_builder.field("storage",
                                                              &&(*__self_0_0));
                                let _ =
                                    debug_trait_builder.field("align",
                                                              &&(*__self_0_1));
                                debug_trait_builder.finish()
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::default::Default,
                      Align: ::core::default::Default>
                 ::core::default::Default for
                 __BindgenBitfieldUnit<Storage, Align> where
                 Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    fn default() -> __BindgenBitfieldUnit<Storage, Align> {
                        __BindgenBitfieldUnit{storage:
                                                  ::core::default::Default::default(),
                                              align:
                                                  ::core::default::Default::default(),}
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::cmp::Eq, Align: ::core::cmp::Eq>
                 ::core::cmp::Eq for __BindgenBitfieldUnit<Storage, Align>
                 where Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    #[doc(hidden)]
                    fn assert_receiver_is_total_eq(&self) -> () {
                        {
                            let _: ::core::cmp::AssertParamIsEq<Storage>;
                            let _: ::core::cmp::AssertParamIsEq<[Align; 0]>;
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::hash::Hash, Align: ::core::hash::Hash>
                 ::core::hash::Hash for __BindgenBitfieldUnit<Storage, Align>
                 where Storage: AsRef<[u8]> + AsMut<[u8]> {
                    fn hash<__HStorageAlign: ::core::hash::Hasher>(&self,
                                                                   state:
                                                                       &mut __HStorageAlign)
                     -> () {
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            => {
                                ::core::hash::Hash::hash(&(*__self_0_0),
                                                         state);
                                ::core::hash::Hash::hash(&(*__self_0_1),
                                                         state)
                            }
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::cmp::Ord, Align: ::core::cmp::Ord>
                 ::core::cmp::Ord for __BindgenBitfieldUnit<Storage, Align>
                 where Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    fn cmp(&self,
                           other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> ::core::cmp::Ordering {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                            &(*__self_1_0)) {
                                    ::core::cmp::Ordering::Equal =>
                                    match ::core::cmp::Ord::cmp(&(*__self_0_1),
                                                                &(*__self_1_1))
                                        {
                                        ::core::cmp::Ordering::Equal =>
                                        ::core::cmp::Ordering::Equal,
                                        cmp => cmp,
                                    },
                                    cmp => cmp,
                                },
                            },
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::cmp::PartialEq,
                      Align: ::core::cmp::PartialEq> ::core::cmp::PartialEq
                 for __BindgenBitfieldUnit<Storage, Align> where
                 Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    fn eq(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                (*__self_0_0) == (*__self_1_0) &&
                                    (*__self_0_1) == (*__self_1_1),
                            },
                        }
                    }
                    #[inline]
                    fn ne(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                (*__self_0_0) != (*__self_1_0) ||
                                    (*__self_0_1) != (*__self_1_1),
                            },
                        }
                    }
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl <Storage: ::core::cmp::PartialOrd,
                      Align: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd
                 for __BindgenBitfieldUnit<Storage, Align> where
                 Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    fn partial_cmp(&self,
                                   other:
                                       &__BindgenBitfieldUnit<Storage, Align>)
                     -> ::core::option::Option<::core::cmp::Ordering> {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                           &(*__self_1_0))
                                    {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                    =>
                                    match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                               &(*__self_1_1))
                                        {
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                        =>
                                        ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                        cmp => cmp,
                                    },
                                    cmp => cmp,
                                },
                            },
                        }
                    }
                    #[inline]
                    fn lt(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                        &(*__self_1_0)),
                                                                                                   ::core::cmp::Ordering::Equal),
                                                                 ||
                                                                     ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                            &(*__self_1_1)),
                                                                                                       ::core::cmp::Ordering::Greater))
                                    == ::core::cmp::Ordering::Less,
                            },
                        }
                    }
                    #[inline]
                    fn le(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                        &(*__self_1_0)),
                                                                                                   ::core::cmp::Ordering::Equal),
                                                                 ||
                                                                     ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                            &(*__self_1_1)),
                                                                                                       ::core::cmp::Ordering::Greater))
                                    != ::core::cmp::Ordering::Greater,
                            },
                        }
                    }
                    #[inline]
                    fn gt(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                        &(*__self_1_0)),
                                                                                                   ::core::cmp::Ordering::Equal),
                                                                 ||
                                                                     ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                            &(*__self_1_1)),
                                                                                                       ::core::cmp::Ordering::Less))
                                    == ::core::cmp::Ordering::Greater,
                            },
                        }
                    }
                    #[inline]
                    fn ge(&self,
                          other: &__BindgenBitfieldUnit<Storage, Align>)
                     -> bool {
                        match *other {
                            __BindgenBitfieldUnit {
                            storage: ref __self_1_0, align: ref __self_1_1 }
                            =>
                            match *self {
                                __BindgenBitfieldUnit {
                                storage: ref __self_0_0, align: ref __self_0_1
                                } =>
                                ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                        &(*__self_1_0)),
                                                                                                   ::core::cmp::Ordering::Equal),
                                                                 ||
                                                                     ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                            &(*__self_1_1)),
                                                                                                       ::core::cmp::Ordering::Less))
                                    != ::core::cmp::Ordering::Less,
                            },
                        }
                    }
                }
                impl <Storage, Align> __BindgenBitfieldUnit<Storage, Align>
                 where Storage: AsRef<[u8]> + AsMut<[u8]> {
                    #[inline]
                    pub fn new(storage: Storage) -> Self {
                        Self{storage, align: [],}
                    }
                    #[inline]
                    pub fn get_bit(&self, index: usize) -> bool {
                        if false {
                            if !(index / 8 < self.storage.as_ref().len()) {
                                {
                                    ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               25u32, 9u32))
                                }
                            };
                        };
                        let byte_index = index / 8;
                        let byte = self.storage.as_ref()[byte_index];
                        let bit_index =
                            if false { 7 - (index % 8) } else { index % 8 };
                        let mask = 1 << bit_index;
                        byte & mask == mask
                    }
                    #[inline]
                    pub fn set_bit(&mut self, index: usize, val: bool) {
                        if false {
                            if !(index / 8 < self.storage.as_ref().len()) {
                                {
                                    ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               38u32, 9u32))
                                }
                            };
                        };
                        let byte_index = index / 8;
                        let byte = &mut self.storage.as_mut()[byte_index];
                        let bit_index =
                            if false { 7 - (index % 8) } else { index % 8 };
                        let mask = 1 << bit_index;
                        if val { *byte |= mask; } else { *byte &= !mask; }
                    }
                    #[inline]
                    pub fn get(&self, bit_offset: usize, bit_width: u8)
                     -> u64 {
                        if false {
                            if !(bit_width <= 64) {
                                {
                                    ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               55u32, 9u32))
                                }
                            };
                        };
                        if false {
                            if !(bit_offset / 8 < self.storage.as_ref().len())
                               {
                                {
                                    ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               56u32, 9u32))
                                }
                            };
                        };
                        if false {
                            if !((bit_offset + (bit_width as usize)) / 8 <=
                                     self.storage.as_ref().len()) {
                                {
                                    ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               57u32, 9u32))
                                }
                            };
                        };
                        let mut val = 0;
                        for i in 0..(bit_width as usize) {
                            if self.get_bit(i + bit_offset) {
                                let index =
                                    if false {
                                        bit_width as usize - 1 - i
                                    } else { i };
                                val |= 1 << index;
                            }
                        }
                        val
                    }
                    #[inline]
                    pub fn set(&mut self, bit_offset: usize, bit_width: u8,
                               val: u64) {
                        if false {
                            if !(bit_width <= 64) {
                                {
                                    ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               73u32, 9u32))
                                }
                            };
                        };
                        if false {
                            if !(bit_offset / 8 < self.storage.as_ref().len())
                               {
                                {
                                    ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               74u32, 9u32))
                                }
                            };
                        };
                        if false {
                            if !((bit_offset + (bit_width as usize)) / 8 <=
                                     self.storage.as_ref().len()) {
                                {
                                    ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                               "src/mynewt/hw/sensor/bindings.rs",
                                                               75u32, 9u32))
                                }
                            };
                        };
                        for i in 0..(bit_width as usize) {
                            let mask = 1 << i;
                            let val_bit_is_set = val & mask == mask;
                            let index =
                                if false {
                                    bit_width as usize - 1 - i
                                } else { i };
                            self.set_bit(index + bit_offset, val_bit_is_set);
                        }
                    }
                }
                #[repr(C)]
                pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
                impl <T> __BindgenUnionField<T> {
                    #[inline]
                    pub fn new() -> Self {
                        __BindgenUnionField(::core::marker::PhantomData)
                    }
                    #[inline]
                    pub unsafe fn as_ref(&self) -> &T {
                        ::core::mem::transmute(self)
                    }
                    #[inline]
                    pub unsafe fn as_mut(&mut self) -> &mut T {
                        ::core::mem::transmute(self)
                    }
                }
                impl <T> ::core::default::Default for __BindgenUnionField<T> {
                    #[inline]
                    fn default() -> Self { Self::new() }
                }
                impl <T> ::core::clone::Clone for __BindgenUnionField<T> {
                    #[inline]
                    fn clone(&self) -> Self { Self::new() }
                }
                impl <T> ::core::marker::Copy for __BindgenUnionField<T> { }
                impl <T> ::core::fmt::Debug for __BindgenUnionField<T> {
                    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
                     -> ::core::fmt::Result {
                        fmt.write_str("__BindgenUnionField")
                    }
                }
                impl <T> ::core::hash::Hash for __BindgenUnionField<T> {
                    fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) {
                    }
                }
                impl <T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
                    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
                        true
                    }
                }
                impl <T> ::core::cmp::Eq for __BindgenUnionField<T> { }
                pub const SENSOR_VALUE_TYPE_OPAQUE: u32 = 0;
                pub const SENSOR_VALUE_TYPE_INT32: u32 = 1;
                pub const SENSOR_VALUE_TYPE_FLOAT: u32 = 2;
                pub const SENSOR_VALUE_TYPE_INT32_TRIPLET: u32 = 3;
                pub const SENSOR_VALUE_TYPE_FLOAT_TRIPLET: u32 = 4;
                pub const SENSOR_ITF_SPI: u32 = 0;
                pub const SENSOR_ITF_I2C: u32 = 1;
                pub const SENSOR_ITF_UART: u32 = 2;
                pub const SENSOR_THRESH_ALGO_WINDOW: u32 = 1;
                pub const SENSOR_THRESH_ALGO_WATERMARK: u32 = 2;
                pub const SENSOR_THRESH_ALGO_USERDEF: u32 = 3;
                pub const SENSOR_IGN_LISTENER: u32 = 1;
                pub type __uint8_t = ::cty::c_uchar;
                pub type __int16_t = ::cty::c_short;
                pub type __uint16_t = ::cty::c_ushort;
                pub type __int32_t = ::cty::c_long;
                pub type __uint32_t = ::cty::c_ulong;
                pub type __int64_t = ::cty::c_longlong;
                pub type os_stack_t = u32;
                pub type os_time_t = u32;
                pub type os_event_fn
                    =
                    ::core::option::Option<unsafe extern "C" fn(ev:
                                                                    *mut os_event)>;
                #[repr(C)]
                pub struct os_event__bindgen_ty_1 {
                    pub stqe_next: *mut os_event,
                }
                impl Default for os_event__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_eventq__bindgen_ty_1 {
                    pub stqh_first: *mut os_event,
                    pub stqh_last: *mut *mut os_event,
                }
                impl Default for os_eventq__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_callout__bindgen_ty_1 {
                    pub tqe_next: *mut os_callout,
                    pub tqe_prev: *mut *mut os_callout,
                }
                impl Default for os_callout__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc = " Initialize a device."]
                #[doc = ""]
                #[doc = " - __`dev`__: The device to initialize."]
                #[doc =
                      " - __`arg`__: User defined argument to pass to the device initalization"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type os_dev_init_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_dev,
                                                                arg2:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                pub type os_dev_open_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_dev,
                                                                arg2: u32,
                                                                arg3:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                pub type os_dev_suspend_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_dev,
                                                                arg2:
                                                                    os_time_t,
                                                                arg3:
                                                                    ::cty::c_int)
                                               -> ::cty::c_int>;
                pub type os_dev_resume_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_dev)
                                               -> ::cty::c_int>;
                pub type os_dev_close_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_dev)
                                               -> ::cty::c_int>;
                #[repr(C)]
                pub struct os_dev__bindgen_ty_1 {
                    pub stqe_next: *mut os_dev,
                }
                impl Default for os_dev__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_memblock__bindgen_ty_1 {
                    pub sle_next: *mut os_memblock,
                }
                impl Default for os_memblock__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_mempool__bindgen_ty_1 {
                    pub stqe_next: *mut os_mempool,
                }
                impl Default for os_mempool__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_mempool__bindgen_ty_2 {
                    pub slh_first: *mut os_memblock,
                }
                impl Default for os_mempool__bindgen_ty_2 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                pub type os_membuf_t = u32;
                #[repr(C)]
                pub struct os_mutex__bindgen_ty_1 {
                    pub slh_first: *mut os_task,
                }
                impl Default for os_mutex__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                pub type os_sanity_check_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut os_sanity_check,
                                                                arg2:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                #[repr(C)]
                pub struct os_sanity_check__bindgen_ty_1 {
                    pub sle_next: *mut os_sanity_check,
                }
                impl Default for os_sanity_check__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                pub type os_task_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut ::cty::c_void)>;
                #[repr(C)]
                pub struct os_task__bindgen_ty_1 {
                    pub stqe_next: *mut os_task,
                }
                impl Default for os_task__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_task__bindgen_ty_2 {
                    pub tqe_next: *mut os_task,
                    pub tqe_prev: *mut *mut os_task,
                }
                impl Default for os_task__bindgen_ty_2 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct os_task__bindgen_ty_3 {
                    pub sle_next: *mut os_task,
                }
                impl Default for os_task__bindgen_ty_3 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                extern "C" {
                    #[doc =
                          " Package init function. Remove when we have post-kernel init stages."]
                    pub fn sensor_pkg_init();
                }
                pub const sensor_type_t_SENSOR_TYPE_NONE: sensor_type_t = 0;
                pub const sensor_type_t_SENSOR_TYPE_ACCELEROMETER:
                          sensor_type_t =
                    1;
                pub const sensor_type_t_SENSOR_TYPE_MAGNETIC_FIELD:
                          sensor_type_t =
                    2;
                pub const sensor_type_t_SENSOR_TYPE_GYROSCOPE: sensor_type_t =
                    4;
                pub const sensor_type_t_SENSOR_TYPE_LIGHT: sensor_type_t = 8;
                pub const sensor_type_t_SENSOR_TYPE_TEMPERATURE: sensor_type_t
                          =
                    16;
                pub const sensor_type_t_SENSOR_TYPE_AMBIENT_TEMPERATURE:
                          sensor_type_t =
                    32;
                pub const sensor_type_t_SENSOR_TYPE_PRESSURE: sensor_type_t =
                    64;
                pub const sensor_type_t_SENSOR_TYPE_PROXIMITY: sensor_type_t =
                    128;
                pub const sensor_type_t_SENSOR_TYPE_RELATIVE_HUMIDITY:
                          sensor_type_t =
                    256;
                pub const sensor_type_t_SENSOR_TYPE_ROTATION_VECTOR:
                          sensor_type_t =
                    512;
                pub const sensor_type_t_SENSOR_TYPE_ALTITUDE: sensor_type_t =
                    1024;
                pub const sensor_type_t_SENSOR_TYPE_WEIGHT: sensor_type_t =
                    2048;
                pub const sensor_type_t_SENSOR_TYPE_LINEAR_ACCEL:
                          sensor_type_t =
                    4096;
                pub const sensor_type_t_SENSOR_TYPE_GRAVITY: sensor_type_t =
                    8192;
                pub const sensor_type_t_SENSOR_TYPE_EULER: sensor_type_t =
                    16384;
                pub const sensor_type_t_SENSOR_TYPE_COLOR: sensor_type_t =
                    32768;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_1:
                          sensor_type_t =
                    67108864;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_2:
                          sensor_type_t =
                    134217728;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_3:
                          sensor_type_t =
                    268435456;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_4:
                          sensor_type_t =
                    536870912;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_5:
                          sensor_type_t =
                    1073741824;
                pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_6:
                          sensor_type_t =
                    -2147483648;
                pub const sensor_type_t_SENSOR_TYPE_ALL: sensor_type_t =
                    4294967295;
                pub type sensor_type_t = i64;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_DOUBLE_TAP:
                          sensor_event_type_t =
                    1;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SINGLE_TAP:
                          sensor_event_type_t =
                    2;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_FREE_FALL:
                          sensor_event_type_t =
                    4;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP_CHANGE:
                          sensor_event_type_t =
                    8;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_WAKEUP:
                          sensor_event_type_t =
                    16;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP:
                          sensor_event_type_t =
                    32;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_CHANGE:
                          sensor_event_type_t =
                    64;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_CHANGE:
                          sensor_event_type_t =
                    128;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_CHANGE:
                          sensor_event_type_t =
                    256;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_CHANGE:
                          sensor_event_type_t =
                    512;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_L_CHANGE:
                          sensor_event_type_t =
                    1024;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_L_CHANGE:
                          sensor_event_type_t =
                    2048;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_L_CHANGE:
                          sensor_event_type_t =
                    4096;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_H_CHANGE:
                          sensor_event_type_t =
                    8192;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_H_CHANGE:
                          sensor_event_type_t =
                    16384;
                pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_H_CHANGE:
                          sensor_event_type_t =
                    32768;
                pub type sensor_event_type_t = u32;
                #[doc =
                      " Configuration structure, describing a specific sensor type off of"]
                #[doc = " an existing sensor."]
                #[repr(C)]
                pub struct sensor_cfg {
                    pub sc_valtype: u8,
                    pub _reserved: [u8; 3usize],
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_cfg {
                    #[inline]
                    fn default() -> sensor_cfg {
                        sensor_cfg{sc_valtype:
                                       ::core::default::Default::default(),
                                   _reserved:
                                       ::core::default::Default::default(),}
                    }
                }
                #[repr(C)]
                pub struct sensor_data_t {
                    pub smd: __BindgenUnionField<*mut sensor_mag_data>,
                    pub sad: __BindgenUnionField<*mut sensor_accel_data>,
                    pub sed: __BindgenUnionField<*mut sensor_euler_data>,
                    pub sqd: __BindgenUnionField<*mut sensor_quat_data>,
                    pub slad: __BindgenUnionField<*mut sensor_accel_data>,
                    pub sgrd: __BindgenUnionField<*mut sensor_accel_data>,
                    pub sgd: __BindgenUnionField<*mut sensor_gyro_data>,
                    pub std: __BindgenUnionField<*mut sensor_temp_data>,
                    pub satd: __BindgenUnionField<*mut sensor_temp_data>,
                    pub sld: __BindgenUnionField<*mut sensor_light_data>,
                    pub scd: __BindgenUnionField<*mut sensor_color_data>,
                    pub spd: __BindgenUnionField<*mut sensor_press_data>,
                    pub srhd: __BindgenUnionField<*mut sensor_humid_data>,
                    pub bindgen_union_field: u64,
                }
                impl Default for sensor_data_t {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc =
                      " Callback for handling sensor data, specified in a sensor listener."]
                #[doc = ""]
                #[doc =
                      " - __`sensor`__: The sensor for which data is being returned"]
                #[doc =
                      " - __`arg`__: The argument provided to sensor_read() function."]
                #[doc =
                      " - __`data`__: A single sensor reading for that sensor listener"]
                #[doc =
                      " - __`type`__: The sensor type for the data function"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_data_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    *mut ::cty::c_void,
                                                                arg3:
                                                                    *mut ::cty::c_void,
                                                                arg4:
                                                                    sensor_type_t)
                                               -> ::cty::c_int>;
                #[doc = " Callback for sending trigger notification."]
                #[doc = ""]
                #[doc = " - __`sensor`__: Ptr to the sensor"]
                #[doc = " - __`data`__: Ptr to sensor data"]
                #[doc = " - __`type`__: The sensor type"]
                pub type sensor_trigger_notify_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    *mut ::cty::c_void,
                                                                arg3:
                                                                    sensor_type_t)
                                               -> ::cty::c_int>;
                #[doc = " Callback for trigger compare functions."]
                #[doc = ""]
                #[doc = " - __`type`__: Type of sensor"]
                #[doc = " - __`low_thresh`__: The sensor low threshold"]
                #[doc = " - __`high_thresh`__: The sensor high threshold"]
                #[doc = " - __`arg`__: Ptr to data"]
                pub type sensor_trigger_cmp_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    sensor_type_t,
                                                                arg2:
                                                                    *mut sensor_data_t,
                                                                arg3:
                                                                    *mut sensor_data_t,
                                                                arg4:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                #[doc = " Callback for event notifications."]
                #[doc = ""]
                #[doc = " - __`sensor`__: The sensor that observed the event"]
                #[doc =
                      " - __`arg`__: The opaque argument provided during registration"]
                #[doc =
                      " - __`event`__: The sensor event type that was observed"]
                pub type sensor_notifier_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    *mut ::cty::c_void,
                                                                arg3:
                                                                    sensor_event_type_t)
                                               -> ::cty::c_int>;
                #[doc = " Callback for reporting a sensor read error."]
                #[doc = ""]
                #[doc =
                      " - __`sensor`__: The sensor for which a read failed."]
                #[doc =
                      " - __`arg`__: The optional argument registered with the callback."]
                #[doc =
                      " - __`status`__: Indicates the cause of the read failure.  Determined by the"]
                #[doc = "               underlying sensor driver."]
                pub type sensor_error_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                    *mut sensor,
                                                                arg:
                                                                    *mut ::cty::c_void,
                                                                status:
                                                                    ::cty::c_int)>;
                #[repr(C)]
                pub struct sensor_listener {
                    pub sl_sensor_type: sensor_type_t,
                    pub sl_func: sensor_data_func_t,
                    pub sl_arg: *mut ::cty::c_void,
                    pub sl_next: sensor_listener__bindgen_ty_1,
                }
                #[repr(C)]
                pub struct sensor_listener__bindgen_ty_1 {
                    pub sle_next: *mut sensor_listener,
                }
                impl Default for sensor_listener__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                impl Default for sensor_listener {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc = " Registration for sensor event notifications"]
                #[repr(C)]
                pub struct sensor_notifier {
                    pub sn_sensor_event_type: sensor_event_type_t,
                    pub sn_func: sensor_notifier_func_t,
                    pub sn_arg: *mut ::cty::c_void,
                    pub sn_next: sensor_notifier__bindgen_ty_1,
                }
                #[repr(C)]
                pub struct sensor_notifier__bindgen_ty_1 {
                    pub sle_next: *mut sensor_notifier,
                }
                impl Default for sensor_notifier__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                impl Default for sensor_notifier {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc = " Context for sensor read events"]
                #[repr(C)]
                pub struct sensor_read_ev_ctx {
                    pub srec_sensor: *mut sensor,
                    pub srec_type: sensor_type_t,
                }
                impl Default for sensor_read_ev_ctx {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc = " Sensor type traits list"]
                #[repr(C)]
                pub struct sensor_type_traits {
                    pub stt_sensor_type: sensor_type_t,
                    pub stt_low_thresh: sensor_data_t,
                    pub stt_high_thresh: sensor_data_t,
                    pub stt_algo: u8,
                    pub stt_poll_n: u16,
                    pub stt_polls_left: u16,
                    pub stt_trigger_cmp_algo: sensor_trigger_cmp_func_t,
                    pub stt_sensor: *mut sensor,
                    pub stt_next: sensor_type_traits__bindgen_ty_1,
                }
                #[repr(C)]
                pub struct sensor_type_traits__bindgen_ty_1 {
                    pub sle_next: *mut sensor_type_traits,
                }
                impl Default for sensor_type_traits__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                impl Default for sensor_type_traits {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor_notify_ev_ctx {
                    pub snec_sensor: *mut sensor,
                    pub snec_evtype: sensor_event_type_t,
                }
                impl Default for sensor_notify_ev_ctx {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor_notify_os_ev {
                    pub snoe_evt: os_event,
                    pub snoe_evtype: sensor_event_type_t,
                    pub snoe_sensor: *mut sensor,
                }
                impl Default for sensor_notify_os_ev {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc =
                      " Read a single value from a sensor, given a specific sensor type"]
                #[doc = " (e.g. SENSOR_TYPE_PROXIMITY)."]
                #[doc = ""]
                #[doc = " - __`sensor`__: The sensor to read from"]
                #[doc =
                      " - __`type`__: The type(s) of sensor values to read.  Mask containing that type, provide"]
                #[doc = "        all, to get all values."]
                #[doc =
                      " - __`data_func`__: The function to call with each value read.  If NULL, it calls all"]
                #[doc =
                      "        sensor listeners associated with this function."]
                #[doc =
                      " - __`arg`__: The argument to pass to the read callback."]
                #[doc =
                      " - __`timeout`__: Timeout. If block until result, specify OS_TIMEOUT_NEVER, 0 returns"]
                #[doc = "        immediately (no wait.)"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_read_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    sensor_type_t,
                                                                arg3:
                                                                    sensor_data_func_t,
                                                                arg4:
                                                                    *mut ::cty::c_void,
                                                                arg5: u32)
                                               -> ::cty::c_int>;
                #[doc =
                      " Get the configuration of the sensor for the sensor type.  This includes"]
                #[doc = " the value type of the sensor."]
                #[doc = ""]
                #[doc = " - __`sensor`__: Ptr to the sensor"]
                #[doc =
                      " - __`type`__: The type of sensor value to get configuration for"]
                #[doc =
                      " - __`cfg`__: A pointer to the sensor value to place the returned result into."]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_get_config_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    sensor_type_t,
                                                                arg3:
                                                                    *mut sensor_cfg)
                                               -> ::cty::c_int>;
                #[doc =
                      " Send a new configuration register set to the sensor."]
                #[doc = ""]
                #[doc =
                      " - __`sensor`__: Ptr to the sensor-specific stucture"]
                #[doc =
                      " - __`arg`__: Ptr to the sensor-specific configuration structure"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_set_config_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                #[doc =
                      " Set the trigger and threshold values for a specific sensor for the sensor"]
                #[doc = " type."]
                #[doc = ""]
                #[doc = " - __`sensor`__: Ptr to the sensor"]
                #[doc = " - __`type`__: type of sensor"]
                #[doc = " - __`stt`__: Ptr to teh sensor traits"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_set_trigger_thresh_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    sensor_type_t,
                                                                stt:
                                                                    *mut sensor_type_traits)
                                               -> ::cty::c_int>;
                #[doc =
                      " Clear the high/low threshold values for a specific sensor for the sensor"]
                #[doc = " type."]
                #[doc = ""]
                #[doc = " - __`sensor`__: Ptr to the sensor"]
                #[doc = " - __`type`__: Type of sensor"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_clear_trigger_thresh_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                    *mut sensor,
                                                                type_:
                                                                    sensor_type_t)
                                               -> ::cty::c_int>;
                #[doc =
                      " Set the notification expectation for a targeted set of events for the"]
                #[doc =
                      " specific sensor. After this function returns successfully, the implementer"]
                #[doc =
                      " shall post corresponding event notifications to the sensor manager."]
                #[doc = ""]
                #[doc =
                      " - __`sensor`__: The sensor to expect notifications from."]
                #[doc =
                      " - __`event`__: The mask of event types to expect notifications from."]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_set_notification_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    sensor_event_type_t)
                                               -> ::cty::c_int>;
                #[doc =
                      " Unset the notification expectation for a targeted set of events for the"]
                #[doc = " specific sensor."]
                #[doc = ""]
                #[doc = " - __`sensor`__: The sensor."]
                #[doc = " - __`event`__: The mask of event types."]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_unset_notification_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    sensor_event_type_t)
                                               -> ::cty::c_int>;
                #[doc = " Let driver handle interrupt in the sensor context"]
                #[doc = ""]
                #[doc = " - __`sensor`__: Ptr to the sensor"]
                #[doc = ""]
                #[doc =
                      " Return: 0 on success, non-zero error code on failure."]
                pub type sensor_handle_interrupt_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                    *mut sensor)
                                               -> ::cty::c_int>;
                #[doc = " Reset Sensor function Ptr"]
                #[doc = ""]
                #[doc = " - __`Ptr`__: to the sensor"]
                #[doc = ""]
                #[doc = " Return: 0 on success, non-zero on failure"]
                pub type sensor_reset_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor)
                                               -> ::cty::c_int>;
                #[repr(C)]
                pub struct sensor_driver {
                    pub sd_read: sensor_read_func_t,
                    pub sd_get_config: sensor_get_config_func_t,
                    pub sd_set_config: sensor_set_config_func_t,
                    pub sd_set_trigger_thresh: sensor_set_trigger_thresh_t,
                    pub sd_clear_low_trigger_thresh: sensor_clear_trigger_thresh_t,
                    pub sd_clear_high_trigger_thresh: sensor_clear_trigger_thresh_t,
                    pub sd_set_notification: sensor_set_notification_t,
                    pub sd_unset_notification: sensor_unset_notification_t,
                    pub sd_handle_interrupt: sensor_handle_interrupt_t,
                    pub sd_reset: sensor_reset_t,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_driver {
                    #[inline]
                    fn default() -> sensor_driver {
                        sensor_driver{sd_read:
                                          ::core::default::Default::default(),
                                      sd_get_config:
                                          ::core::default::Default::default(),
                                      sd_set_config:
                                          ::core::default::Default::default(),
                                      sd_set_trigger_thresh:
                                          ::core::default::Default::default(),
                                      sd_clear_low_trigger_thresh:
                                          ::core::default::Default::default(),
                                      sd_clear_high_trigger_thresh:
                                          ::core::default::Default::default(),
                                      sd_set_notification:
                                          ::core::default::Default::default(),
                                      sd_unset_notification:
                                          ::core::default::Default::default(),
                                      sd_handle_interrupt:
                                          ::core::default::Default::default(),
                                      sd_reset:
                                          ::core::default::Default::default(),}
                    }
                }
                #[repr(C)]
                pub struct sensor_timestamp {
                    pub st_ostv: os_timeval,
                    pub st_ostz: os_timezone,
                    pub st_cputime: u32,
                }
                impl Default for sensor_timestamp {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor_int {
                    pub host_pin: u8,
                    pub device_pin: u8,
                    pub active: u8,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_int {
                    #[inline]
                    fn default() -> sensor_int {
                        sensor_int{host_pin:
                                       ::core::default::Default::default(),
                                   device_pin:
                                       ::core::default::Default::default(),
                                   active:
                                       ::core::default::Default::default(),}
                    }
                }
                #[repr(C)]
                pub struct sensor_itf {
                    pub si_type: u8,
                    pub si_num: u8,
                    pub si_cs_pin: u8,
                    pub si_addr: u16,
                    pub si_lock: *mut os_mutex,
                    pub si_low_pin: u8,
                    pub si_high_pin: u8,
                    pub si_ints: [sensor_int; 2usize],
                }
                impl Default for sensor_itf {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor {
                    pub s_dev: *mut os_dev,
                    pub s_lock: os_mutex,
                    pub s_types: sensor_type_t,
                    pub s_mask: sensor_type_t,
                    #[doc = " Poll rate in MS for this sensor."]
                    pub s_poll_rate: u32,
                    pub s_next_run: os_time_t,
                    pub s_funcs: *mut sensor_driver,
                    pub s_sts: sensor_timestamp,
                    pub s_itf: sensor_itf,
                    pub s_interrupt_evt: os_event,
                    pub s_listener_list: sensor__bindgen_ty_1,
                    pub s_err_fn: sensor_error_func_t,
                    pub s_err_arg: *mut ::cty::c_void,
                    pub s_notifier_list: sensor__bindgen_ty_2,
                    pub s_type_traits_list: sensor__bindgen_ty_3,
                    pub s_next: sensor__bindgen_ty_4,
                }
                #[repr(C)]
                pub struct sensor__bindgen_ty_1 {
                    pub slh_first: *mut sensor_listener,
                }
                impl Default for sensor__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor__bindgen_ty_2 {
                    pub slh_first: *mut sensor_notifier,
                }
                impl Default for sensor__bindgen_ty_2 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor__bindgen_ty_3 {
                    pub slh_first: *mut sensor_type_traits,
                }
                impl Default for sensor__bindgen_ty_3 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[repr(C)]
                pub struct sensor__bindgen_ty_4 {
                    pub sle_next: *mut sensor,
                }
                impl Default for sensor__bindgen_ty_4 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                impl Default for sensor {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                #[doc =
                      " Read context for calling user function with argument"]
                #[repr(C)]
                pub struct sensor_read_ctx {
                    pub user_func: sensor_data_func_t,
                    pub user_arg: *mut ::cty::c_void,
                }
                impl Default for sensor_read_ctx {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                extern "C" {
                    #[doc =
                          " Lock access to the sensor_itf specified by si.  Blocks until lock acquired."]
                    #[doc = ""]
                    #[doc = " - __`si`__: The sensor_itf to lock"]
                    #[doc = " - __`timeout`__: The timeout"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure."]
                    pub fn sensor_itf_lock(si: *mut sensor_itf,
                                           timeout: os_time_t)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Unlock access to the sensor_itf specified by si."]
                    #[doc = ""]
                    #[doc = " - __`si`__: The sensor_itf to unlock access to"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure."]
                    pub fn sensor_itf_unlock(si: *mut sensor_itf);
                }
                extern "C" {
                    #[doc = " Initialize a sensor"]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor to initialize"]
                    #[doc =
                          " - __`dev`__: The device to associate with this sensor."]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_init(sensor: *mut sensor, dev: *mut os_dev)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Lock access to the sensor specified by sensor.  Blocks until lock acquired."]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor to lock"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure."]
                    pub fn sensor_lock(sensor: *mut sensor) -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Unlock access to the sensor specified by sensor."]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: The sensor to unlock access to."]
                    pub fn sensor_unlock(sensor: *mut sensor);
                }
                extern "C" {
                    #[doc =
                          " Register a sensor listener. This allows a calling application to receive"]
                    #[doc = " callbacks for data from a given sensor object."]
                    #[doc = ""]
                    #[doc =
                          " For more information on the type of callbacks available, see the documentation"]
                    #[doc = " for the sensor listener structure."]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: The sensor to register a listener on"]
                    #[doc =
                          " - __`listener`__: The listener to register onto the sensor"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_register_listener(sensor: *mut sensor,
                                                    listener:
                                                        *mut sensor_listener)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Un-register a sensor listener. This allows a calling application to clear"]
                    #[doc = " callbacks for a given sensor object."]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor object"]
                    #[doc =
                          " - __`listener`__: The listener to remove from the sensor listener list"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_unregister_listener(sensor: *mut sensor,
                                                      listener:
                                                          *mut sensor_listener)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Register a sensor error callback.  The callback is executed when the sensor"]
                    #[doc = " manager fails to read from the given sensor."]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: The sensor to register an error callback on."]
                    #[doc =
                          " - __`err_fn`__: The function to execute when a read fails."]
                    #[doc =
                          " - __`arg`__: Optional argument to pass to the callback."]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_register_err_func(sensor: *mut sensor,
                                                    err_fn:
                                                        sensor_error_func_t,
                                                    arg: *mut ::cty::c_void)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Register a sensor notifier. This allows a calling application to receive"]
                    #[doc =
                          " callbacks any time a requested event is observed."]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: The sensor to register the notifier on"]
                    #[doc = " - __`notifier`__: The notifier to register"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_register_notifier(sensor: *mut sensor,
                                                    notifier:
                                                        *mut sensor_notifier)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Un-register a sensor notifier. This allows a calling application to stop"]
                    #[doc =
                          " receiving callbacks for events on the sensor object."]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: The sensor object to un-register the notifier on"]
                    #[doc =
                          " - __`notifier`__: The notifier to remove from the notification list"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_unregister_notifier(sensor: *mut sensor,
                                                      notifier:
                                                          *mut sensor_notifier)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Read the data for sensor type \"type,\" from the given sensor and"]
                    #[doc =
                          " return the result into the \"value\" parameter."]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor to read data from"]
                    #[doc =
                          " - __`type`__: The type of sensor data to read from the sensor"]
                    #[doc =
                          " - __`data_func`__: The callback to call for data returned from that sensor"]
                    #[doc =
                          " - __`arg`__: The argument to pass to this callback."]
                    #[doc =
                          " - __`timeout`__: Timeout before aborting sensor read"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure."]
                    pub fn sensor_read(sensor: *mut sensor,
                                       type_: sensor_type_t,
                                       data_func: sensor_data_func_t,
                                       arg: *mut ::cty::c_void, timeout: u32)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Lock sensor manager to access the list of sensors"]
                    pub fn sensor_mgr_lock() -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Unlock sensor manager once the list of sensors has been accessed"]
                    pub fn sensor_mgr_unlock();
                }
                extern "C" {
                    #[doc =
                          " Register the sensor with the global sensor list. This makes the sensor"]
                    #[doc =
                          " searchable by other packages, who may want to look it up by type."]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor to register"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure."]
                    pub fn sensor_mgr_register(sensor: *mut sensor)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Get the current eventq, the system is misconfigured if there is still"]
                    #[doc = " no parent eventq."]
                    #[doc = ""]
                    #[doc =
                          " Return: Ptr OS eventq that the sensor mgr is set to"]
                    pub fn sensor_mgr_evq_get() -> *mut os_eventq;
                }
                pub type sensor_mgr_compare_func_t
                    =
                    ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut sensor,
                                                                arg2:
                                                                    *mut ::cty::c_void)
                                               -> ::cty::c_int>;
                extern "C" {
                    #[doc =
                          " The sensor manager contains a list of sensors, this function returns"]
                    #[doc =
                          " the next sensor in that list, for which compare_func() returns successful"]
                    #[doc =
                          " (one).  If prev_cursor is provided, the function starts at that point"]
                    #[doc = " in the sensor list."]
                    #[doc = ""]
                    #[doc =
                          " @warn This function MUST be locked by sensor_mgr_lock/unlock() if the goal is"]
                    #[doc =
                          " to iterate through sensors (as opposed to just finding one.)  As the"]
                    #[doc =
                          " \"prev_cursor\" may be resorted in the sensor list, in between calls."]
                    #[doc = ""]
                    #[doc =
                          " - __`compare_func`__: The comparison function to use against sensors in the list."]
                    #[doc =
                          " - __`arg`__: The argument to provide to that comparison function"]
                    #[doc =
                          " - __`prev_cursor`__: The previous sensor in the sensor manager list, in case of"]
                    #[doc =
                          "        iteration.  If desire is to find first matching sensor, provide a"]
                    #[doc = "        NULL value."]
                    #[doc = ""]
                    #[doc =
                          " Return: A pointer to the first sensor found from prev_cursor, or"]
                    #[doc = "         NULL, if none found."]
                    #[doc = ""]
                    pub fn sensor_mgr_find_next(arg1:
                                                    sensor_mgr_compare_func_t,
                                                arg2: *mut ::cty::c_void,
                                                arg3: *mut sensor)
                     -> *mut sensor;
                }
                extern "C" {
                    #[doc =
                          " Find the \"next\" sensor available for a given sensor type."]
                    #[doc = ""]
                    #[doc =
                          " If the sensor parameter, is present find the next entry from that"]
                    #[doc =
                          " parameter.  Otherwise, find the first matching sensor."]
                    #[doc = ""]
                    #[doc = " - __`type`__: The type of sensor to search for"]
                    #[doc =
                          " - __`sensor`__: The cursor to search from, or NULL to start from the beginning."]
                    #[doc = ""]
                    #[doc =
                          " Return: A pointer to the sensor object matching that sensor type, or NULL if"]
                    #[doc = "         none found."]
                    pub fn sensor_mgr_find_next_bytype(type_: sensor_type_t,
                                                       sensor: *mut sensor)
                     -> *mut sensor;
                }
                extern "C" {
                    #[doc =
                          " Search the sensor list and find the next sensor that corresponds"]
                    #[doc = " to a given device name."]
                    #[doc = ""]
                    #[doc = " - __`devname`__: The device name to search for"]
                    #[doc =
                          " - __`sensor`__: The previous sensor found with this device name"]
                    #[doc = ""]
                    #[doc =
                          " Return: 0 on success, non-zero error code on failure"]
                    pub fn sensor_mgr_find_next_bydevname(devname:
                                                              *const ::cty::c_char,
                                                          prev_cursor:
                                                              *mut sensor)
                     -> *mut sensor;
                }
                extern "C" {
                    #[doc = " Check if sensor type matches"]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: The sensor object"]
                    #[doc = " - __`arg`__: type to check"]
                    #[doc = ""]
                    #[doc = " Return: 1 if matches, 0 if it doesn't match."]
                    pub fn sensor_mgr_match_bytype(sensor: *mut sensor,
                                                   arg1: *mut ::cty::c_void)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc = " Set the sensor poll rate"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc =
                          " - __`poll_rate`__: The poll rate in milli seconds"]
                    pub fn sensor_set_poll_rate_ms(devname:
                                                       *const ::cty::c_char,
                                                   poll_rate: u32)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Set the sensor poll rate multiple based on the device name, sensor type"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc = " - __`stt`__: The sensor type trait"]
                    pub fn sensor_set_n_poll_rate(devname:
                                                      *const ::cty::c_char,
                                                  stt:
                                                      *mut sensor_type_traits)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc = " Transmit OIC trigger"]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: Ptr to the sensor"]
                    #[doc = " - __`arg`__: Ptr to sensor data"]
                    #[doc = " - __`type`__: The sensor type"]
                    #[doc = ""]
                    #[doc = " Return: 0 on sucess, non-zero on failure"]
                    pub fn sensor_oic_tx_trigger(sensor: *mut sensor,
                                                 arg: *mut ::cty::c_void,
                                                 type_: sensor_type_t)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc = " Sensor trigger initialization"]
                    #[doc = ""]
                    #[doc = " - __`sensor`__: Ptr to the sensor"]
                    #[doc =
                          " - __`type`__: Sensor type to enable trigger for"]
                    #[doc =
                          " - __`notify`__: the function to call if the trigger condition is satisfied"]
                    pub fn sensor_trigger_init(sensor: *mut sensor,
                                               type_: sensor_type_t,
                                               notify:
                                                   sensor_trigger_notify_func_t);
                }
                extern "C" {
                    #[doc =
                          " Search the sensor type traits list for specific type of sensor"]
                    #[doc = ""]
                    #[doc = " - __`type`__: The sensor type to search for"]
                    #[doc = " - __`sensor`__: Ptr to a sensor"]
                    #[doc = ""]
                    #[doc =
                          " Return: NULL when no sensor type is found, ptr to sensor_type_traits structure"]
                    #[doc = " when found"]
                    pub fn sensor_get_type_traits_bytype(type_: sensor_type_t,
                                                         sensor: *mut sensor)
                     -> *mut sensor_type_traits;
                }
                extern "C" {
                    #[doc = " Get the type traits for a sensor"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc = " - __`stt`__: Ptr to sensor types trait struct"]
                    #[doc = " - __`type`__: The sensor type"]
                    #[doc = ""]
                    #[doc =
                          " Return: NULL on failure, sensor struct on success"]
                    pub fn sensor_get_type_traits_byname(arg1:
                                                             *const ::cty::c_char,
                                                         arg2:
                                                             *mut *mut sensor_type_traits,
                                                         arg3: sensor_type_t)
                     -> *mut sensor;
                }
                extern "C" {
                    #[doc =
                          " Set the thresholds along with the comparison algo for a sensor"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc =
                          " - __`stt`__: Ptr to sensor type traits containing thresholds"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure"]
                    pub fn sensor_set_thresh(devname: *const ::cty::c_char,
                                             stt: *mut sensor_type_traits)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc = " Clears the low threshold for a sensor"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc = " - __`type`__: The sensor type"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure"]
                    pub fn sensor_clear_low_thresh(devname:
                                                       *const ::cty::c_char,
                                                   type_: sensor_type_t)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc = " Clears the high threshold for a sensor"]
                    #[doc = ""]
                    #[doc = " - __`devname`__: Name of the sensor"]
                    #[doc = " - __`type`__: The sensor type"]
                    #[doc = ""]
                    #[doc = " Return: 0 on success, non-zero on failure"]
                    pub fn sensor_clear_high_thresh(devname:
                                                        *const ::cty::c_char,
                                                    type_: sensor_type_t)
                     -> ::cty::c_int;
                }
                extern "C" {
                    #[doc =
                          " Puts a notification event on the sensor manager evq"]
                    #[doc = ""]
                    #[doc = " - __`ctx`__: Notification event context"]
                    #[doc = " - __`evtype`__: The notification event type"]
                    pub fn sensor_mgr_put_notify_evt(ctx:
                                                         *mut sensor_notify_ev_ctx,
                                                     evtype:
                                                         sensor_event_type_t);
                }
                extern "C" {
                    #[doc =
                          " Puts a interrupt event on the sensor manager evq"]
                    #[doc = ""]
                    #[doc =
                          " - __`sensor`__: Sensor Ptr as interrupt event context"]
                    pub fn sensor_mgr_put_interrupt_evt(sensor: *mut sensor);
                }
                extern "C" {
                    #[doc = " Puts read event on the sensor manager evq"]
                    #[doc = ""]
                    #[doc = " - __`arg`__: Argument"]
                    pub fn sensor_mgr_put_read_evt(arg: *mut ::cty::c_void);
                }
                extern "C" {
                    #[doc = " Resets the sensor"]
                    #[doc = ""]
                    #[doc = " - __`Ptr`__: to sensor"]
                    pub fn sensor_reset(sensor: *mut sensor) -> ::cty::c_int;
                }
                #[repr(C, packed)]
                pub struct sensor_accel_data {
                    pub sad_x: f32,
                    pub sad_y: f32,
                    pub sad_z: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_accel_data {
                    #[inline]
                    fn default() -> sensor_accel_data {
                        sensor_accel_data{sad_x:
                                              ::core::default::Default::default(),
                                          sad_y:
                                              ::core::default::Default::default(),
                                          sad_z:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_accel_data {
                    #[inline]
                    pub fn sad_x_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sad_x_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sad_y_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sad_y_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sad_z_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sad_z_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(sad_x_is_valid: u8,
                                          sad_y_is_valid: u8,
                                          sad_z_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let sad_x_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sad_x_is_valid)
                                                            };
                                                        sad_x_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let sad_y_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sad_y_is_valid)
                                                            };
                                                        sad_y_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let sad_z_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sad_z_is_valid)
                                                            };
                                                        sad_z_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_mag_data {
                    pub smd_x: f32,
                    pub smd_y: f32,
                    pub smd_z: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_mag_data {
                    #[inline]
                    fn default() -> sensor_mag_data {
                        sensor_mag_data{smd_x:
                                            ::core::default::Default::default(),
                                        smd_y:
                                            ::core::default::Default::default(),
                                        smd_z:
                                            ::core::default::Default::default(),
                                        _bitfield_1:
                                            ::core::default::Default::default(),}
                    }
                }
                impl sensor_mag_data {
                    #[inline]
                    pub fn smd_x_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_smd_x_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn smd_y_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_smd_y_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn smd_z_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_smd_z_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(smd_x_is_valid: u8,
                                          smd_y_is_valid: u8,
                                          smd_z_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let smd_x_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(smd_x_is_valid)
                                                            };
                                                        smd_x_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let smd_y_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(smd_y_is_valid)
                                                            };
                                                        smd_y_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let smd_z_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(smd_z_is_valid)
                                                            };
                                                        smd_z_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_light_data {
                    pub sld_full: u16,
                    pub sld_ir: u16,
                    pub sld_lux: u32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_light_data {
                    #[inline]
                    fn default() -> sensor_light_data {
                        sensor_light_data{sld_full:
                                              ::core::default::Default::default(),
                                          sld_ir:
                                              ::core::default::Default::default(),
                                          sld_lux:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_light_data {
                    #[inline]
                    pub fn sld_full_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sld_full_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sld_ir_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sld_ir_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sld_lux_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sld_lux_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(sld_full_is_valid: u8,
                                          sld_ir_is_valid: u8,
                                          sld_lux_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let sld_full_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sld_full_is_valid)
                                                            };
                                                        sld_full_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let sld_ir_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sld_ir_is_valid)
                                                            };
                                                        sld_ir_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let sld_lux_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sld_lux_is_valid)
                                                            };
                                                        sld_lux_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_quat_data {
                    pub sqd_x: f32,
                    pub sqd_y: f32,
                    pub sqd_z: f32,
                    pub sqd_w: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_quat_data {
                    #[inline]
                    fn default() -> sensor_quat_data {
                        sensor_quat_data{sqd_x:
                                             ::core::default::Default::default(),
                                         sqd_y:
                                             ::core::default::Default::default(),
                                         sqd_z:
                                             ::core::default::Default::default(),
                                         sqd_w:
                                             ::core::default::Default::default(),
                                         _bitfield_1:
                                             ::core::default::Default::default(),}
                    }
                }
                impl sensor_quat_data {
                    #[inline]
                    pub fn sqd_x_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sqd_x_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sqd_y_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sqd_y_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sqd_z_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sqd_z_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sqd_w_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(3usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sqd_w_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(3usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(sqd_x_is_valid: u8,
                                          sqd_y_is_valid: u8,
                                          sqd_z_is_valid: u8,
                                          sqd_w_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let sqd_x_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sqd_x_is_valid)
                                                            };
                                                        sqd_x_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let sqd_y_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sqd_y_is_valid)
                                                            };
                                                        sqd_y_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let sqd_z_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sqd_z_is_valid)
                                                            };
                                                        sqd_z_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(3usize, 1u8,
                                                    {
                                                        let sqd_w_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sqd_w_is_valid)
                                                            };
                                                        sqd_w_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_euler_data {
                    pub sed_h: f32,
                    pub sed_r: f32,
                    pub sed_p: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_euler_data {
                    #[inline]
                    fn default() -> sensor_euler_data {
                        sensor_euler_data{sed_h:
                                              ::core::default::Default::default(),
                                          sed_r:
                                              ::core::default::Default::default(),
                                          sed_p:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_euler_data {
                    #[inline]
                    pub fn sed_h_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sed_h_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sed_r_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sed_r_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sed_p_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sed_p_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(sed_h_is_valid: u8,
                                          sed_r_is_valid: u8,
                                          sed_p_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let sed_h_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sed_h_is_valid)
                                                            };
                                                        sed_h_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let sed_r_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sed_r_is_valid)
                                                            };
                                                        sed_r_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let sed_p_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sed_p_is_valid)
                                                            };
                                                        sed_p_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_color_data {
                    pub scd_r: u16,
                    pub scd_g: u16,
                    pub scd_b: u16,
                    pub scd_c: u16,
                    pub scd_lux: u16,
                    pub scd_colortemp: u16,
                    pub scd_saturation: u16,
                    pub scd_saturation75: u16,
                    pub scd_is_sat: u8,
                    pub scd_cratio: f32,
                    pub scd_maxlux: u16,
                    pub scd_ir: u16,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 2usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_color_data {
                    #[inline]
                    fn default() -> sensor_color_data {
                        sensor_color_data{scd_r:
                                              ::core::default::Default::default(),
                                          scd_g:
                                              ::core::default::Default::default(),
                                          scd_b:
                                              ::core::default::Default::default(),
                                          scd_c:
                                              ::core::default::Default::default(),
                                          scd_lux:
                                              ::core::default::Default::default(),
                                          scd_colortemp:
                                              ::core::default::Default::default(),
                                          scd_saturation:
                                              ::core::default::Default::default(),
                                          scd_saturation75:
                                              ::core::default::Default::default(),
                                          scd_is_sat:
                                              ::core::default::Default::default(),
                                          scd_cratio:
                                              ::core::default::Default::default(),
                                          scd_maxlux:
                                              ::core::default::Default::default(),
                                          scd_ir:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_color_data {
                    #[inline]
                    pub fn scd_r_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_r_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_g_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_g_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_b_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_b_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_c_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(3usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_c_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(3usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_lux_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(4usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_lux_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(4usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_colortemp_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(5usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_colortemp_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(5usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_saturation_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(6usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_saturation_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(6usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_saturation75_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(7usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_saturation75_is_valid(&mut self,
                                                         val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(7usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_is_sat_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(8usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_is_sat_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(8usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_cratio_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(9usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_cratio_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(9usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_maxlux_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(10usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_maxlux_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(10usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn scd_ir_is_valid(&self) -> u16 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(11usize,
                                                                        1u8)
                                                       as u16)
                        }
                    }
                    #[inline]
                    pub fn set_scd_ir_is_valid(&mut self, val: u16) {
                        unsafe {
                            let val: u16 = ::core::mem::transmute(val);
                            self._bitfield_1.set(11usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(scd_r_is_valid: u16,
                                          scd_g_is_valid: u16,
                                          scd_b_is_valid: u16,
                                          scd_c_is_valid: u16,
                                          scd_lux_is_valid: u16,
                                          scd_colortemp_is_valid: u16,
                                          scd_saturation_is_valid: u16,
                                          scd_saturation75_is_valid: u16,
                                          scd_is_sat_is_valid: u16,
                                          scd_cratio_is_valid: u16,
                                          scd_maxlux_is_valid: u16,
                                          scd_ir_is_valid: u16)
                     -> __BindgenBitfieldUnit<[u8; 2usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 2usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let scd_r_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_r_is_valid)
                                                            };
                                                        scd_r_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let scd_g_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_g_is_valid)
                                                            };
                                                        scd_g_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let scd_b_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_b_is_valid)
                                                            };
                                                        scd_b_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(3usize, 1u8,
                                                    {
                                                        let scd_c_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_c_is_valid)
                                                            };
                                                        scd_c_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(4usize, 1u8,
                                                    {
                                                        let scd_lux_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_lux_is_valid)
                                                            };
                                                        scd_lux_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit.set(5usize, 1u8,
                                                    {
                                                        let scd_colortemp_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_colortemp_is_valid)
                                                            };
                                                        scd_colortemp_is_valid
                                                            as u64
                                                    });
                        __bindgen_bitfield_unit.set(6usize, 1u8,
                                                    {
                                                        let scd_saturation_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_saturation_is_valid)
                                                            };
                                                        scd_saturation_is_valid
                                                            as u64
                                                    });
                        __bindgen_bitfield_unit.set(7usize, 1u8,
                                                    {
                                                        let scd_saturation75_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_saturation75_is_valid)
                                                            };
                                                        scd_saturation75_is_valid
                                                            as u64
                                                    });
                        __bindgen_bitfield_unit.set(8usize, 1u8,
                                                    {
                                                        let scd_is_sat_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_is_sat_is_valid)
                                                            };
                                                        scd_is_sat_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit.set(9usize, 1u8,
                                                    {
                                                        let scd_cratio_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_cratio_is_valid)
                                                            };
                                                        scd_cratio_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit.set(10usize, 1u8,
                                                    {
                                                        let scd_maxlux_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_maxlux_is_valid)
                                                            };
                                                        scd_maxlux_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit.set(11usize, 1u8,
                                                    {
                                                        let scd_ir_is_valid:
                                                                u16 =
                                                            unsafe {
                                                                ::core::mem::transmute(scd_ir_is_valid)
                                                            };
                                                        scd_ir_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_temp_data {
                    pub std_temp: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_temp_data {
                    #[inline]
                    fn default() -> sensor_temp_data {
                        sensor_temp_data{std_temp:
                                             ::core::default::Default::default(),
                                         _bitfield_1:
                                             ::core::default::Default::default(),}
                    }
                }
                impl sensor_temp_data {
                    #[inline]
                    pub fn std_temp_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_std_temp_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(std_temp_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let std_temp_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(std_temp_is_valid)
                                                            };
                                                        std_temp_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_press_data {
                    pub spd_press: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_press_data {
                    #[inline]
                    fn default() -> sensor_press_data {
                        sensor_press_data{spd_press:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_press_data {
                    #[inline]
                    pub fn spd_press_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_spd_press_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(spd_press_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let spd_press_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(spd_press_is_valid)
                                                            };
                                                        spd_press_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_humid_data {
                    pub shd_humid: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_humid_data {
                    #[inline]
                    fn default() -> sensor_humid_data {
                        sensor_humid_data{shd_humid:
                                              ::core::default::Default::default(),
                                          _bitfield_1:
                                              ::core::default::Default::default(),}
                    }
                }
                impl sensor_humid_data {
                    #[inline]
                    pub fn shd_humid_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_shd_humid_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(shd_humid_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let shd_humid_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(shd_humid_is_valid)
                                                            };
                                                        shd_humid_is_valid as
                                                            u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C, packed)]
                pub struct sensor_gyro_data {
                    pub sgd_x: f32,
                    pub sgd_y: f32,
                    pub sgd_z: f32,
                    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
                }
                #[automatically_derived]
                #[allow(unused_qualifications)]
                impl ::core::default::Default for sensor_gyro_data {
                    #[inline]
                    fn default() -> sensor_gyro_data {
                        sensor_gyro_data{sgd_x:
                                             ::core::default::Default::default(),
                                         sgd_y:
                                             ::core::default::Default::default(),
                                         sgd_z:
                                             ::core::default::Default::default(),
                                         _bitfield_1:
                                             ::core::default::Default::default(),}
                    }
                }
                impl sensor_gyro_data {
                    #[inline]
                    pub fn sgd_x_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sgd_x_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(0usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sgd_y_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(1usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sgd_y_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(1usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn sgd_z_is_valid(&self) -> u8 {
                        unsafe {
                            ::core::mem::transmute(self._bitfield_1.get(2usize,
                                                                        1u8)
                                                       as u8)
                        }
                    }
                    #[inline]
                    pub fn set_sgd_z_is_valid(&mut self, val: u8) {
                        unsafe {
                            let val: u8 = ::core::mem::transmute(val);
                            self._bitfield_1.set(2usize, 1u8, val as u64)
                        }
                    }
                    #[inline]
                    pub fn new_bitfield_1(sgd_x_is_valid: u8,
                                          sgd_y_is_valid: u8,
                                          sgd_z_is_valid: u8)
                     -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                        let mut __bindgen_bitfield_unit:
                                __BindgenBitfieldUnit<[u8; 1usize], u8> =
                            Default::default();
                        __bindgen_bitfield_unit.set(0usize, 1u8,
                                                    {
                                                        let sgd_x_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sgd_x_is_valid)
                                                            };
                                                        sgd_x_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(1usize, 1u8,
                                                    {
                                                        let sgd_y_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sgd_y_is_valid)
                                                            };
                                                        sgd_y_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit.set(2usize, 1u8,
                                                    {
                                                        let sgd_z_is_valid:
                                                                u8 =
                                                            unsafe {
                                                                ::core::mem::transmute(sgd_z_is_valid)
                                                            };
                                                        sgd_z_is_valid as u64
                                                    });
                        __bindgen_bitfield_unit
                    }
                }
                #[repr(C)]
                pub struct _bindgen_ty_1 {
                    pub mgr_lock: os_mutex,
                    pub mgr_wakeup_callout: os_callout,
                    pub mgr_eventq: *mut os_eventq,
                    pub mgr_sensor_list: _bindgen_ty_1__bindgen_ty_1,
                }
                #[repr(C)]
                pub struct _bindgen_ty_1__bindgen_ty_1 {
                    pub slh_first: *mut sensor,
                }
                impl Default for _bindgen_ty_1__bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                impl Default for _bindgen_ty_1 {
                    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
                }
                extern "C" {
                    pub static mut sensor_mgr: _bindgen_ty_1;
                }
                extern "C" {
                    pub static mut sensor_base_ts: sensor_timestamp;
                }
                extern "C" {
                    pub static mut sensor_read_event: os_event;
                }
                extern "C" {
                    pub static mut sensor_notify_evt_pool: os_mempool;
                }
                extern "C" {
                    pub static mut sensor_notify_evt_area:
                               [os_membuf_t; 60usize];
                }
            }
            /// Export all bindings. TODO: Export only the API bindings.
            pub use self::bindings::*;
            /// Points to a `sensor`.  Needed because `sensor` also refers to a namespace.
            pub type sensor_ptr = *mut sensor;
            /// Points to sensor arg passed by Mynewt to sensor listener
            pub type sensor_arg = *mut c_void;
            /// Points to sensor data passed by Mynewt to sensor listener
            pub type sensor_data_ptr = *mut c_void;
            /// Sensor data function that returns `MynewtError` instead of `i32`
            pub type sensor_data_func
                =
                unsafe extern "C" fn(sensor: sensor_ptr, arg: sensor_arg,
                                     data: sensor_data_ptr,
                                     stype: sensor_type_t) -> MynewtError;
            /// Sensor data function that returns `i32` instead of `MynewtError`
            pub type sensor_data_func_untyped
                =
                unsafe extern "C" fn(sensor: sensor_ptr, arg: sensor_arg,
                                     data: sensor_data_ptr,
                                     stype: sensor_type_t) -> i32;
            /// Cast sensor data function from typed to untyped
            pub fn as_untyped(typed: sensor_data_func)
             -> Option<sensor_data_func_untyped> {
                let untyped =
                    unsafe {
                        ::core::mem::transmute::<sensor_data_func,
                                                 sensor_data_func_untyped>(typed)
                    };
                Some(untyped)
            }
            ///  Register a sensor listener. This allows a calling application to receive
            ///  callbacks for data from a given sensor object. This is the safe version of `sensor_register_listener()`
            ///  that copies the listener locally before passing to Mynewt.
            ///  For more information on the type of callbacks available, see the documentation
            ///  for the sensor listener structure.
            ///  `sensor`: The sensor to register a listener on.
            ///  `listener`: The listener to register onto the sensor.
            ///  Returns `Ok()` on success, `Err()` containing `MynewtError` error code on failure.
            pub fn register_listener(sensor: *mut sensor,
                                     listener: sensor_listener)
             -> MynewtResult<()> {
                unsafe {
                    if !(LISTENER_INTERNAL.sl_sensor_type == 0) {
                        {
                            ::core::panicking::panic(&("assertion failed: LISTENER_INTERNAL.sl_sensor_type == 0",
                                                       "src/mynewt/hw/sensor.rs",
                                                       66u32, 14u32))
                        }
                    }
                };
                unsafe { LISTENER_INTERNAL = listener };
                unsafe {
                    sensor_register_listener(sensor, &mut LISTENER_INTERNAL)
                };
                Ok(())
            }
            ///  Define the listener function to be called after polling the sensor.
            ///  This is a static mutable copy of the listener passed in through `register_listener`.
            ///  Must be static so it won't go out of scope.  Must be mutable so that Rust won't move it while Mynewt is using it.
            static mut LISTENER_INTERNAL: sensor_listener =
                sensor_listener{sl_func:
                                    Some(null_sensor_data_func),
                                                                   ..unsafe {
                                                                         ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_listener>()],
                                                                                                  sensor_listener>([0;
                                                                                                                       ::core::mem::size_of::<sensor_listener>()])
                                                                     }};
            ///  Define a default sensor data function in case there is none.
            extern "C" fn null_sensor_data_func(_sensor: sensor_ptr,
                                                _arg: sensor_arg,
                                                _sensor_data: sensor_data_ptr,
                                                _sensor_type: sensor_type_t)
             -> i32 {
                0
            }
        }
    }
    pub mod libs {
        //! Mynewt Custom API for Rust
        /// Contains Rust bindings for Mynewt Custom API `libs/sensor_coap`
        pub mod sensor_coap {
            use super::super::encoding::json::*;
            #[repr(C)]
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub struct __BindgenBitfieldUnit<Storage, Align> where
                       Storage: AsRef<[u8]> + AsMut<[u8]> {
                storage: Storage,
                align: [Align; 0],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::marker::Copy, Align: ::core::marker::Copy>
             ::core::marker::Copy for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::clone::Clone, Align: ::core::clone::Clone>
             ::core::clone::Clone for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn clone(&self) -> __BindgenBitfieldUnit<Storage, Align> {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } =>
                        __BindgenBitfieldUnit{storage:
                                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                                              align:
                                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::fmt::Debug, Align: ::core::fmt::Debug>
             ::core::fmt::Debug for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter)
                 -> ::core::fmt::Result {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            let mut debug_trait_builder =
                                f.debug_struct("__BindgenBitfieldUnit");
                            let _ =
                                debug_trait_builder.field("storage",
                                                          &&(*__self_0_0));
                            let _ =
                                debug_trait_builder.field("align",
                                                          &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::default::Default,
                  Align: ::core::default::Default> ::core::default::Default
             for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn default() -> __BindgenBitfieldUnit<Storage, Align> {
                    __BindgenBitfieldUnit{storage:
                                              ::core::default::Default::default(),
                                          align:
                                              ::core::default::Default::default(),}
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Eq, Align: ::core::cmp::Eq>
             ::core::cmp::Eq for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Storage>;
                        let _: ::core::cmp::AssertParamIsEq<[Align; 0]>;
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::hash::Hash, Align: ::core::hash::Hash>
             ::core::hash::Hash for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn hash<__HStorageAlign: ::core::hash::Hasher>(&self,
                                                               state:
                                                                   &mut __HStorageAlign)
                 -> () {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            ::core::hash::Hash::hash(&(*__self_0_0), state);
                            ::core::hash::Hash::hash(&(*__self_0_1), state)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Ord, Align: ::core::cmp::Ord>
             ::core::cmp::Ord for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn cmp(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::cmp::Ordering {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                        &(*__self_1_0)) {
                                ::core::cmp::Ordering::Equal =>
                                match ::core::cmp::Ord::cmp(&(*__self_0_1),
                                                            &(*__self_1_1)) {
                                    ::core::cmp::Ordering::Equal =>
                                    ::core::cmp::Ordering::Equal,
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialEq,
                  Align: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn eq(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialOrd,
                  Align: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn partial_cmp(&self,
                               other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::option::Option<::core::cmp::Ordering> {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                       &(*__self_1_0))
                                {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                =>
                                match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                           &(*__self_1_1))
                                    {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                    =>
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
                #[inline]
                fn lt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                == ::core::cmp::Ordering::Less,
                        },
                    }
                }
                #[inline]
                fn le(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                != ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn gt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                == ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn ge(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                != ::core::cmp::Ordering::Less,
                        },
                    }
                }
            }
            impl <Storage, Align> __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                pub fn new(storage: Storage) -> Self {
                    Self{storage, align: [],}
                }
                #[inline]
                pub fn get_bit(&self, index: usize) -> bool {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           25u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = self.storage.as_ref()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    byte & mask == mask
                }
                #[inline]
                pub fn set_bit(&mut self, index: usize, val: bool) {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           38u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = &mut self.storage.as_mut()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    if val { *byte |= mask; } else { *byte &= !mask; }
                }
                #[inline]
                pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           55u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           56u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           57u32, 9u32))
                            }
                        };
                    };
                    let mut val = 0;
                    for i in 0..(bit_width as usize) {
                        if self.get_bit(i + bit_offset) {
                            let index =
                                if false {
                                    bit_width as usize - 1 - i
                                } else { i };
                            val |= 1 << index;
                        }
                    }
                    val
                }
                #[inline]
                pub fn set(&mut self, bit_offset: usize, bit_width: u8,
                           val: u64) {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           73u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           74u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/libs/sensor_coap.rs",
                                                           75u32, 9u32))
                            }
                        };
                    };
                    for i in 0..(bit_width as usize) {
                        let mask = 1 << i;
                        let val_bit_is_set = val & mask == mask;
                        let index =
                            if false {
                                bit_width as usize - 1 - i
                            } else { i };
                        self.set_bit(index + bit_offset, val_bit_is_set);
                    }
                }
            }
            #[repr(C)]
            pub struct __IncompleteArrayField<T>(::core::marker::PhantomData<T>,
                                                 [T; 0]);
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <T: ::core::default::Default> ::core::default::Default for
             __IncompleteArrayField<T> {
                #[inline]
                fn default() -> __IncompleteArrayField<T> {
                    __IncompleteArrayField(::core::default::Default::default(),
                                           ::core::default::Default::default())
                }
            }
            impl <T> __IncompleteArrayField<T> {
                #[inline]
                pub fn new() -> Self {
                    __IncompleteArrayField(::core::marker::PhantomData, [])
                }
                #[inline]
                pub unsafe fn as_ptr(&self) -> *const T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_slice(&self, len: usize) -> &[T] {
                    ::core::slice::from_raw_parts(self.as_ptr(), len)
                }
                #[inline]
                pub unsafe fn as_mut_slice(&mut self, len: usize)
                 -> &mut [T] {
                    ::core::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
                }
            }
            impl <T> ::core::fmt::Debug for __IncompleteArrayField<T> {
                fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
                 -> ::core::fmt::Result {
                    fmt.write_str("__IncompleteArrayField")
                }
            }
            impl <T> ::core::clone::Clone for __IncompleteArrayField<T> {
                #[inline]
                fn clone(&self) -> Self { Self::new() }
            }
            #[repr(C)]
            pub struct __BindgenUnionField<T>(::core::marker::PhantomData<T>);
            impl <T> __BindgenUnionField<T> {
                #[inline]
                pub fn new() -> Self {
                    __BindgenUnionField(::core::marker::PhantomData)
                }
                #[inline]
                pub unsafe fn as_ref(&self) -> &T {
                    ::core::mem::transmute(self)
                }
                #[inline]
                pub unsafe fn as_mut(&mut self) -> &mut T {
                    ::core::mem::transmute(self)
                }
            }
            impl <T> ::core::default::Default for __BindgenUnionField<T> {
                #[inline]
                fn default() -> Self { Self::new() }
            }
            impl <T> ::core::clone::Clone for __BindgenUnionField<T> {
                #[inline]
                fn clone(&self) -> Self { Self::new() }
            }
            impl <T> ::core::marker::Copy for __BindgenUnionField<T> { }
            impl <T> ::core::fmt::Debug for __BindgenUnionField<T> {
                fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>)
                 -> ::core::fmt::Result {
                    fmt.write_str("__BindgenUnionField")
                }
            }
            impl <T> ::core::hash::Hash for __BindgenUnionField<T> {
                fn hash<H: ::core::hash::Hasher>(&self, _state: &mut H) { }
            }
            impl <T> ::core::cmp::PartialEq for __BindgenUnionField<T> {
                fn eq(&self, _other: &__BindgenUnionField<T>) -> bool { true }
            }
            impl <T> ::core::cmp::Eq for __BindgenUnionField<T> { }
            pub const COAP_LINK_FORMAT_FILTERING: u32 = 0;
            pub const COAP_PROXY_OPTION_PROCESSING: u32 = 0;
            pub const COAP_MAX_ATTEMPTS: u32 = 2;
            pub const COAP_OBSERVE_REFRESH_INTERVAL: u32 = 20;
            pub const COAP_DEFAULT_PORT: u32 = 5683;
            pub const COAP_DEFAULT_MAX_AGE: u32 = 60;
            pub const COAP_RESPONSE_RANDOM_FACTOR: f64 = 1.5;
            pub const COAP_MAX_RETRANSMIT: u32 = 4;
            pub const COAP_HEADER_LEN: u32 = 4;
            pub const COAP_TOKEN_LEN: u32 = 8;
            pub const COAP_ETAG_LEN: u32 = 8;
            pub const COAP_MAX_URI: u32 = 32;
            pub const COAP_MAX_URI_QUERY: u32 = 32;
            pub const COAP_TCP_LENGTH8_OFF: u32 = 13;
            pub const COAP_TCP_LENGTH16_OFF: u32 = 269;
            pub const COAP_TCP_LENGTH32_OFF: u32 = 65805;
            pub const COAP_TCP_TYPE0: u32 = 0;
            pub const COAP_TCP_TYPE8: u32 = 13;
            pub const COAP_TCP_TYPE16: u32 = 14;
            pub const COAP_TCP_TYPE32: u32 = 15;
            pub const COAP_HEADER_OPTION_DELTA_MASK: u32 = 240;
            pub const COAP_HEADER_OPTION_SHORT_LENGTH_MASK: u32 = 15;
            pub const COAP_PORT_UNSECURED: u32 = 5683;
            pub type __uint8_t = ::cty::c_uchar;
            pub type __uint16_t = ::cty::c_ushort;
            pub type __uint32_t = ::cty::c_ulong;
            pub type __uint64_t = ::cty::c_ulonglong;
            #[doc =
                  " A mbuf pool from which to allocate mbufs. This contains a pointer to the os"]
            #[doc =
                  " mempool to allocate mbufs out of, the total number of elements in the pool,"]
            #[doc =
                  " and the amount of \"user\" data in a non-packet header mbuf. The total pool"]
            #[doc = " size, in bytes, should be:"]
            #[doc =
                  "  os_mbuf_count * (omp_databuf_len + sizeof(struct os_mbuf))"]
            #[repr(C)]
            pub struct os_mbuf_pool {
                #[doc =
                      " Total length of the databuf in each mbuf.  This is the size of the"]
                #[doc = " mempool block, minus the mbuf header"]
                pub omp_databuf_len: u16,
                #[doc = " The memory pool which to allocate mbufs out of"]
                pub omp_pool: *mut os_mempool,
                pub omp_next: os_mbuf_pool__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_mbuf_pool__bindgen_ty_1 {
                pub stqe_next: *mut os_mbuf_pool,
            }
            impl Default for os_mbuf_pool__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mbuf_pool {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Chained memory buffer."]
            #[repr(C)]
            pub struct os_mbuf {
                #[doc = " Current pointer to data in the structure"]
                pub om_data: *mut u8,
                #[doc =
                      " Flags associated with this buffer, see OS_MBUF_F_* defintions"]
                pub om_flags: u8,
                #[doc = " Length of packet header"]
                pub om_pkthdr_len: u8,
                #[doc = " Length of data in this buffer"]
                pub om_len: u16,
                #[doc = " The mbuf pool this mbuf was allocated out of"]
                pub om_omp: *mut os_mbuf_pool,
                pub om_next: os_mbuf__bindgen_ty_1,
                #[doc =
                      " Pointer to the beginning of the data, after this buffer"]
                pub om_databuf: __IncompleteArrayField<u8>,
            }
            #[repr(C)]
            pub struct os_mbuf__bindgen_ty_1 {
                pub sle_next: *mut os_mbuf,
            }
            impl Default for os_mbuf__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mbuf {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc =
                  " A memory block structure. This simply contains a pointer to the free list"]
            #[doc =
                  " chain and is only used when the block is on the free list. When the block"]
            #[doc =
                  " has been removed from the free list the entire memory block is usable by the"]
            #[doc = " caller."]
            #[repr(C)]
            pub struct os_memblock {
                pub mb_next: os_memblock__bindgen_ty_1,
            }
            #[repr(C)]
            pub struct os_memblock__bindgen_ty_1 {
                pub sle_next: *mut os_memblock,
            }
            impl Default for os_memblock__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_memblock {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Memory pool"]
            #[repr(C)]
            pub struct os_mempool {
                #[doc = " Size of the memory blocks, in bytes."]
                pub mp_block_size: u32,
                #[doc = " The number of memory blocks."]
                pub mp_num_blocks: u16,
                #[doc = " The number of free blocks left"]
                pub mp_num_free: u16,
                #[doc = " The lowest number of free blocks seen"]
                pub mp_min_free: u16,
                #[doc = " Bitmap of OS_MEMPOOL_F_[...] values."]
                pub mp_flags: u8,
                #[doc = " Address of memory buffer used by pool"]
                pub mp_membuf_addr: u32,
                pub mp_list: os_mempool__bindgen_ty_1,
                pub __bindgen_anon_1: os_mempool__bindgen_ty_2,
                #[doc = " Name for memory block"]
                pub name: *mut ::cty::c_char,
            }
            #[repr(C)]
            pub struct os_mempool__bindgen_ty_1 {
                pub stqe_next: *mut os_mempool,
            }
            impl Default for os_mempool__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_mempool__bindgen_ty_2 {
                pub slh_first: *mut os_memblock,
            }
            impl Default for os_mempool__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for os_mempool {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C, packed)]
            pub struct oc_ep_hdr {
                pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for oc_ep_hdr {
                #[inline]
                fn default() -> oc_ep_hdr {
                    oc_ep_hdr{_bitfield_1:
                                  ::core::default::Default::default(),}
                }
            }
            impl oc_ep_hdr {
                #[inline]
                pub fn oe_type(&self) -> u8 {
                    unsafe {
                        ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                    3u8) as
                                                   u8)
                    }
                }
                #[inline]
                pub fn set_oe_type(&mut self, val: u8) {
                    unsafe {
                        let val: u8 = ::core::mem::transmute(val);
                        self._bitfield_1.set(0usize, 3u8, val as u64)
                    }
                }
                #[inline]
                pub fn oe_flags(&self) -> u8 {
                    unsafe {
                        ::core::mem::transmute(self._bitfield_1.get(3usize,
                                                                    5u8) as
                                                   u8)
                    }
                }
                #[inline]
                pub fn set_oe_flags(&mut self, val: u8) {
                    unsafe {
                        let val: u8 = ::core::mem::transmute(val);
                        self._bitfield_1.set(3usize, 5u8, val as u64)
                    }
                }
                #[inline]
                pub fn new_bitfield_1(oe_type: u8, oe_flags: u8)
                 -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                    let mut __bindgen_bitfield_unit:
                            __BindgenBitfieldUnit<[u8; 1usize], u8> =
                        Default::default();
                    __bindgen_bitfield_unit.set(0usize, 3u8,
                                                {
                                                    let oe_type: u8 =
                                                        unsafe {
                                                            ::core::mem::transmute(oe_type)
                                                        };
                                                    oe_type as u64
                                                });
                    __bindgen_bitfield_unit.set(3usize, 5u8,
                                                {
                                                    let oe_flags: u8 =
                                                        unsafe {
                                                            ::core::mem::transmute(oe_flags)
                                                        };
                                                    oe_flags as u64
                                                });
                    __bindgen_bitfield_unit
                }
            }
            #[repr(C)]
            pub struct oc_endpoint {
                pub ep: oc_ep_hdr,
                pub _res: [u8; 23usize],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for oc_endpoint {
                #[inline]
                fn default() -> oc_endpoint {
                    oc_endpoint{ep: ::core::default::Default::default(),
                                _res: ::core::default::Default::default(),}
                }
            }
            pub type oc_endpoint_t = oc_endpoint;
            #[repr(C)]
            pub struct stats_coap_stats {
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for stats_coap_stats {
                #[inline]
                fn default() -> stats_coap_stats { stats_coap_stats{} }
            }
            extern "C" {
                pub static mut coap_stats: stats_coap_stats;
            }
            extern "C" {
                pub static mut coap_error_message: *mut ::cty::c_char;
            }
            #[repr(C)]
            pub struct oc_server_handle {
                pub endpoint: oc_endpoint_t,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for oc_server_handle {
                #[inline]
                fn default() -> oc_server_handle {
                    oc_server_handle{endpoint:
                                         ::core::default::Default::default(),}
                }
            }
            #[repr(C)]
            pub struct sensor_value {
                pub key: *const ::cty::c_char,
                pub val_type: ::cty::c_int,
                pub int_val: u16,
                pub float_val: f32,
            }
            impl Default for sensor_value {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc =
                      "  Init the Sensor CoAP module. Called by sysinit() during startup, defined in pkg.yml."]
                pub fn init_sensor_coap();
            }
            extern "C" {
                pub fn sensor_coap_ready() -> bool;
            }
            extern "C" {
                pub fn init_sensor_post(server: *mut oc_server_handle,
                                        uri: *const ::cty::c_char,
                                        coap_content_format: ::cty::c_int)
                 -> bool;
            }
            extern "C" {
                pub fn do_sensor_post() -> bool;
            }
            #[repr(C)]
            pub struct json_value__bindgen_ty_1 {
                pub u: __BindgenUnionField<u64>,
                pub fl: __BindgenUnionField<f32>,
                pub str: __BindgenUnionField<*mut ::cty::c_char>,
                pub composite: __BindgenUnionField<json_value__bindgen_ty_1__bindgen_ty_1>,
                pub bindgen_union_field: [u64; 2usize],
            }
            #[repr(C)]
            pub struct json_value__bindgen_ty_1__bindgen_ty_1 {
                pub keys: *mut *mut ::cty::c_char,
                pub values: *mut *mut json_value,
            }
            impl Default for json_value__bindgen_ty_1__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            impl Default for json_value__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub type json_write_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(buf:
                                                                *mut ::cty::c_void,
                                                            data:
                                                                *mut ::cty::c_char,
                                                            len: ::cty::c_int)
                                           -> ::cty::c_int>;
            extern "C" {
                pub static mut coap_json_encoder: json_encoder;
            }
            extern "C" {
                pub static mut coap_json_value: json_value;
            }
            extern "C" {
                #[doc =
                      "  Prepare to write a new JSON CoAP payload into the mbuf."]
                pub fn json_rep_new(m: *mut os_mbuf);
            }
            extern "C" {
                #[doc =
                      "  Close the current JSON CoAP payload.  Erase the JSON encoder."]
                pub fn json_rep_reset();
            }
            extern "C" {
                #[doc = "  Finalise the payload and return the payload size."]
                pub fn json_rep_finalize() -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      " Start the JSON representation.  Assume top level is object."]
                #[doc = " ```"]
                #[doc = " --> {"]
                #[doc = " ```"]
                pub fn json_rep_start_root_object();
            }
            extern "C" {
                #[doc =
                      "  End the JSON representation.  Assume top level is object."]
                #[doc = "  ```"]
                #[doc = "  {... --> {...}"]
                #[doc = "  ```"]
                pub fn json_rep_end_root_object();
            }
            extern "C" {
                pub static mut coap_json_mbuf: *mut os_mbuf;
            }
        }
        /// Contains Rust bindings for Mynewt Custom API `libs/sensor_network`
        pub mod sensor_network {
            use super::sensor_coap::*;
            pub const SENSOR_NETWORK_SIZE: u32 = 5;
            pub type __uint8_t = ::cty::c_uchar;
            pub type __uint16_t = ::cty::c_ushort;
            #[repr(C)]
            pub struct oc_server_handle {
                _unused: [u8; 0],
            }
            extern "C" {
                pub fn init_sensor_post(server: *mut oc_server_handle,
                                        uri: *const ::cty::c_char,
                                        coap_content_format: ::cty::c_int)
                 -> bool;
            }
            extern "C" {
                pub fn do_sensor_post() -> bool;
            }
            #[repr(C)]
            pub struct sensor_network_interface {
                pub iface_type: u8,
                pub network_device: *const ::cty::c_char,
                pub server_endpoint_size: u8,
                pub register_transport_func: ::core::option::Option<unsafe extern "C" fn(network_device:
                                                                                             *const ::cty::c_char,
                                                                                         server_endpoint:
                                                                                             *mut ::cty::c_void,
                                                                                         host:
                                                                                             *const ::cty::c_char,
                                                                                         port:
                                                                                             u16,
                                                                                         server_endpoint_size:
                                                                                             u8)
                                                                        ->
                                                                            ::cty::c_int>,
                pub transport_registered: u8,
            }
            impl Default for sensor_network_interface {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            extern "C" {
                #[doc = ""]
                pub fn register_server_transport() -> ::cty::c_int;
            }
            extern "C" {
                pub fn register_collector_transport() -> ::cty::c_int;
            }
            extern "C" {
                pub fn sensor_network_register_transport(iface_type: u8)
                 -> ::cty::c_int;
            }
            extern "C" {
                pub fn init_server_post(uri: *const ::cty::c_char) -> bool;
            }
            extern "C" {
                pub fn init_collector_post() -> bool;
            }
            extern "C" {
                pub fn sensor_network_init_post(iface_type: u8,
                                                uri: *const ::cty::c_char)
                 -> bool;
            }
            extern "C" {
                pub fn do_server_post() -> bool;
            }
            extern "C" {
                pub fn do_collector_post() -> bool;
            }
            extern "C" {
                pub fn sensor_network_do_post(iface_type: u8) -> bool;
            }
            extern "C" {
                pub fn is_collector_node() -> bool;
            }
            extern "C" {
                pub fn is_sensor_node() -> bool;
            }
            extern "C" {
                pub fn is_standalone_node() -> bool;
            }
            extern "C" {
                pub fn should_send_to_collector(val: *mut sensor_value,
                                                device_name:
                                                    *const ::cty::c_char)
                 -> bool;
            }
            extern "C" {
                pub fn get_device_id() -> *const ::cty::c_char;
            }
            extern "C" {
                #[doc = ""]
                pub fn sensor_network_init();
            }
            extern "C" {
                pub fn sensor_network_register_interface(iface:
                                                             *const sensor_network_interface)
                 -> ::cty::c_int;
            }
            #[doc = ""]
            #[repr(C)]
            pub struct sensor_network_endpoint {
                pub endpoint: [u8; 16usize],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for sensor_network_endpoint {
                #[inline]
                fn default() -> sensor_network_endpoint {
                    sensor_network_endpoint{endpoint:
                                                ::core::default::Default::default(),}
                }
            }
            extern "C" {
                pub static mut sensor_network_interfaces:
                           [sensor_network_interface; 2usize];
            }
            extern "C" {
                pub static mut sensor_network_endpoints:
                           [sensor_network_endpoint; 2usize];
            }
            extern "C" {
                pub static mut sensor_network_encoding:
                           [::cty::c_int; 2usize];
            }
            extern "C" {
                pub static mut sensor_network_shortname:
                           [*const ::cty::c_char; 2usize];
            }
        }
        /// Contains Rust bindings for Mynewt Custom API `libs/mynewt_rust`
        pub mod mynewt_rust {
            use super::super::kernel::os::*;
            use super::super::hw::sensor::*;
            #[repr(C)]
            #[structural_match]
            #[rustc_copy_clone_marker]
            pub struct __BindgenBitfieldUnit<Storage, Align> where
                       Storage: AsRef<[u8]> + AsMut<[u8]> {
                storage: Storage,
                align: [Align; 0],
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::marker::Copy, Align: ::core::marker::Copy>
             ::core::marker::Copy for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::clone::Clone, Align: ::core::clone::Clone>
             ::core::clone::Clone for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn clone(&self) -> __BindgenBitfieldUnit<Storage, Align> {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } =>
                        __BindgenBitfieldUnit{storage:
                                                  ::core::clone::Clone::clone(&(*__self_0_0)),
                                              align:
                                                  ::core::clone::Clone::clone(&(*__self_0_1)),},
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::fmt::Debug, Align: ::core::fmt::Debug>
             ::core::fmt::Debug for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter)
                 -> ::core::fmt::Result {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            let mut debug_trait_builder =
                                f.debug_struct("__BindgenBitfieldUnit");
                            let _ =
                                debug_trait_builder.field("storage",
                                                          &&(*__self_0_0));
                            let _ =
                                debug_trait_builder.field("align",
                                                          &&(*__self_0_1));
                            debug_trait_builder.finish()
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::default::Default,
                  Align: ::core::default::Default> ::core::default::Default
             for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn default() -> __BindgenBitfieldUnit<Storage, Align> {
                    __BindgenBitfieldUnit{storage:
                                              ::core::default::Default::default(),
                                          align:
                                              ::core::default::Default::default(),}
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Eq, Align: ::core::cmp::Eq>
             ::core::cmp::Eq for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                #[doc(hidden)]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Storage>;
                        let _: ::core::cmp::AssertParamIsEq<[Align; 0]>;
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::hash::Hash, Align: ::core::hash::Hash>
             ::core::hash::Hash for __BindgenBitfieldUnit<Storage, Align>
             where Storage: AsRef<[u8]> + AsMut<[u8]> {
                fn hash<__HStorageAlign: ::core::hash::Hasher>(&self,
                                                               state:
                                                                   &mut __HStorageAlign)
                 -> () {
                    match *self {
                        __BindgenBitfieldUnit {
                        storage: ref __self_0_0, align: ref __self_0_1 } => {
                            ::core::hash::Hash::hash(&(*__self_0_0), state);
                            ::core::hash::Hash::hash(&(*__self_0_1), state)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::Ord, Align: ::core::cmp::Ord>
             ::core::cmp::Ord for __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn cmp(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::cmp::Ordering {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::Ord::cmp(&(*__self_0_0),
                                                        &(*__self_1_0)) {
                                ::core::cmp::Ordering::Equal =>
                                match ::core::cmp::Ord::cmp(&(*__self_0_1),
                                                            &(*__self_1_1)) {
                                    ::core::cmp::Ordering::Equal =>
                                    ::core::cmp::Ordering::Equal,
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialEq,
                  Align: ::core::cmp::PartialEq> ::core::cmp::PartialEq for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn eq(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) == (*__self_1_0) &&
                                (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            (*__self_0_0) != (*__self_1_0) ||
                                (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl <Storage: ::core::cmp::PartialOrd,
                  Align: ::core::cmp::PartialOrd> ::core::cmp::PartialOrd for
             __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                fn partial_cmp(&self,
                               other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> ::core::option::Option<::core::cmp::Ordering> {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                       &(*__self_1_0))
                                {
                                ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                =>
                                match ::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                           &(*__self_1_1))
                                    {
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal)
                                    =>
                                    ::core::option::Option::Some(::core::cmp::Ordering::Equal),
                                    cmp => cmp,
                                },
                                cmp => cmp,
                            },
                        },
                    }
                }
                #[inline]
                fn lt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                == ::core::cmp::Ordering::Less,
                        },
                    }
                }
                #[inline]
                fn le(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Greater))
                                != ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn gt(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                == ::core::cmp::Ordering::Greater,
                        },
                    }
                }
                #[inline]
                fn ge(&self, other: &__BindgenBitfieldUnit<Storage, Align>)
                 -> bool {
                    match *other {
                        __BindgenBitfieldUnit {
                        storage: ref __self_1_0, align: ref __self_1_1 } =>
                        match *self {
                            __BindgenBitfieldUnit {
                            storage: ref __self_0_0, align: ref __self_0_1 }
                            =>
                            ::core::cmp::Ordering::then_with(::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_0),
                                                                                                                                    &(*__self_1_0)),
                                                                                               ::core::cmp::Ordering::Equal),
                                                             ||
                                                                 ::core::option::Option::unwrap_or(::core::cmp::PartialOrd::partial_cmp(&(*__self_0_1),
                                                                                                                                        &(*__self_1_1)),
                                                                                                   ::core::cmp::Ordering::Less))
                                != ::core::cmp::Ordering::Less,
                        },
                    }
                }
            }
            impl <Storage, Align> __BindgenBitfieldUnit<Storage, Align> where
             Storage: AsRef<[u8]> + AsMut<[u8]> {
                #[inline]
                pub fn new(storage: Storage) -> Self {
                    Self{storage, align: [],}
                }
                #[inline]
                pub fn get_bit(&self, index: usize) -> bool {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           27u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = self.storage.as_ref()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    byte & mask == mask
                }
                #[inline]
                pub fn set_bit(&mut self, index: usize, val: bool) {
                    if false {
                        if !(index / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: index / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           40u32, 9u32))
                            }
                        };
                    };
                    let byte_index = index / 8;
                    let byte = &mut self.storage.as_mut()[byte_index];
                    let bit_index =
                        if false { 7 - (index % 8) } else { index % 8 };
                    let mask = 1 << bit_index;
                    if val { *byte |= mask; } else { *byte &= !mask; }
                }
                #[inline]
                pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           57u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           58u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           59u32, 9u32))
                            }
                        };
                    };
                    let mut val = 0;
                    for i in 0..(bit_width as usize) {
                        if self.get_bit(i + bit_offset) {
                            let index =
                                if false {
                                    bit_width as usize - 1 - i
                                } else { i };
                            val |= 1 << index;
                        }
                    }
                    val
                }
                #[inline]
                pub fn set(&mut self, bit_offset: usize, bit_width: u8,
                           val: u64) {
                    if false {
                        if !(bit_width <= 64) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_width <= 64",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           75u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !(bit_offset / 8 < self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: bit_offset / 8 < self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           76u32, 9u32))
                            }
                        };
                    };
                    if false {
                        if !((bit_offset + (bit_width as usize)) / 8 <=
                                 self.storage.as_ref().len()) {
                            {
                                ::core::panicking::panic(&("assertion failed: (bit_offset + (bit_width as usize)) / 8 <= self.storage.as_ref().len()",
                                                           "src/mynewt/libs/mynewt_rust.rs",
                                                           77u32, 9u32))
                            }
                        };
                    };
                    for i in 0..(bit_width as usize) {
                        let mask = 1 << i;
                        let val_bit_is_set = val & mask == mask;
                        let index =
                            if false {
                                bit_width as usize - 1 - i
                            } else { i };
                        self.set_bit(index + bit_offset, val_bit_is_set);
                    }
                }
            }
            pub type __uint8_t = ::cty::c_uchar;
            pub type __int16_t = ::cty::c_short;
            pub type __uint16_t = ::cty::c_ushort;
            pub type __int32_t = ::cty::c_long;
            pub type __uint32_t = ::cty::c_ulong;
            pub type __int64_t = ::cty::c_longlong;
            pub type __uint64_t = ::cty::c_ulonglong;
            extern "C" {
                #[doc =
                      "  Initialise the Mynewt system.  Start the Mynewt drivers and libraries.  Equivalent to `sysinit()` macro in C."]
                pub fn rust_sysinit();
            }
            pub type os_stack_t = u32;
            pub type os_time_t = u32;
            pub type os_event_fn
                =
                ::core::option::Option<unsafe extern "C" fn(ev:
                                                                *mut os_event)>;
            #[repr(C)]
            pub struct os_event__bindgen_ty_1 {
                pub stqe_next: *mut os_event,
            }
            impl Default for os_event__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = " Initialize a device."]
            #[doc = ""]
            #[doc = " - __`dev`__: The device to initialize."]
            #[doc =
                  " - __`arg`__: User defined argument to pass to the device initalization"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type os_dev_init_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            pub type os_dev_open_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2: u32,
                                                            arg3:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            pub type os_dev_suspend_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev,
                                                            arg2: os_time_t,
                                                            arg3:
                                                                ::cty::c_int)
                                           -> ::cty::c_int>;
            pub type os_dev_resume_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev)
                                           -> ::cty::c_int>;
            pub type os_dev_close_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut os_dev)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct os_dev__bindgen_ty_1 {
                pub stqe_next: *mut os_dev,
            }
            impl Default for os_dev__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_mutex__bindgen_ty_1 {
                pub slh_first: *mut os_task,
            }
            impl Default for os_mutex__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub type os_sanity_check_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut os_sanity_check,
                                                            arg2:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct os_sanity_check__bindgen_ty_1 {
                pub sle_next: *mut os_sanity_check,
            }
            impl Default for os_sanity_check__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub type os_task_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                *mut ::cty::c_void)>;
            #[repr(C)]
            pub struct os_task__bindgen_ty_1 {
                pub stqe_next: *mut os_task,
            }
            impl Default for os_task__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_task__bindgen_ty_2 {
                pub tqe_next: *mut os_task,
                pub tqe_prev: *mut *mut os_task,
            }
            impl Default for os_task__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct os_task__bindgen_ty_3 {
                pub sle_next: *mut os_task,
            }
            impl Default for os_task__bindgen_ty_3 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            pub const sensor_type_t_SENSOR_TYPE_NONE: sensor_type_t = 0;
            pub const sensor_type_t_SENSOR_TYPE_ACCELEROMETER: sensor_type_t =
                1;
            pub const sensor_type_t_SENSOR_TYPE_MAGNETIC_FIELD: sensor_type_t
                      =
                2;
            pub const sensor_type_t_SENSOR_TYPE_GYROSCOPE: sensor_type_t = 4;
            pub const sensor_type_t_SENSOR_TYPE_LIGHT: sensor_type_t = 8;
            pub const sensor_type_t_SENSOR_TYPE_TEMPERATURE: sensor_type_t =
                16;
            pub const sensor_type_t_SENSOR_TYPE_AMBIENT_TEMPERATURE:
                      sensor_type_t =
                32;
            pub const sensor_type_t_SENSOR_TYPE_PRESSURE: sensor_type_t = 64;
            pub const sensor_type_t_SENSOR_TYPE_PROXIMITY: sensor_type_t =
                128;
            pub const sensor_type_t_SENSOR_TYPE_RELATIVE_HUMIDITY:
                      sensor_type_t =
                256;
            pub const sensor_type_t_SENSOR_TYPE_ROTATION_VECTOR: sensor_type_t
                      =
                512;
            pub const sensor_type_t_SENSOR_TYPE_ALTITUDE: sensor_type_t =
                1024;
            pub const sensor_type_t_SENSOR_TYPE_WEIGHT: sensor_type_t = 2048;
            pub const sensor_type_t_SENSOR_TYPE_LINEAR_ACCEL: sensor_type_t =
                4096;
            pub const sensor_type_t_SENSOR_TYPE_GRAVITY: sensor_type_t = 8192;
            pub const sensor_type_t_SENSOR_TYPE_EULER: sensor_type_t = 16384;
            pub const sensor_type_t_SENSOR_TYPE_COLOR: sensor_type_t = 32768;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_1: sensor_type_t
                      =
                67108864;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_2: sensor_type_t
                      =
                134217728;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_3: sensor_type_t
                      =
                268435456;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_4: sensor_type_t
                      =
                536870912;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_5: sensor_type_t
                      =
                1073741824;
            pub const sensor_type_t_SENSOR_TYPE_USER_DEFINED_6: sensor_type_t
                      =
                -2147483648;
            pub const sensor_type_t_SENSOR_TYPE_ALL: sensor_type_t =
                4294967295;
            pub type sensor_type_t = i64;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_DOUBLE_TAP:
                      sensor_event_type_t =
                1;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SINGLE_TAP:
                      sensor_event_type_t =
                2;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_FREE_FALL:
                      sensor_event_type_t =
                4;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP_CHANGE:
                      sensor_event_type_t =
                8;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_WAKEUP:
                      sensor_event_type_t =
                16;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_SLEEP:
                      sensor_event_type_t =
                32;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_CHANGE:
                      sensor_event_type_t =
                64;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_CHANGE:
                      sensor_event_type_t =
                128;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_CHANGE:
                      sensor_event_type_t =
                256;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_CHANGE:
                      sensor_event_type_t =
                512;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_L_CHANGE:
                      sensor_event_type_t =
                1024;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_L_CHANGE:
                      sensor_event_type_t =
                2048;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_L_CHANGE:
                      sensor_event_type_t =
                4096;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_X_H_CHANGE:
                      sensor_event_type_t =
                8192;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Y_H_CHANGE:
                      sensor_event_type_t =
                16384;
            pub const sensor_event_type_t_SENSOR_EVENT_TYPE_ORIENT_Z_H_CHANGE:
                      sensor_event_type_t =
                32768;
            pub type sensor_event_type_t = u32;
            #[doc =
                  " Callback for handling sensor data, specified in a sensor listener."]
            #[doc = ""]
            #[doc =
                  " - __`sensor`__: The sensor for which data is being returned"]
            #[doc =
                  " - __`arg`__: The argument provided to sensor_read() function."]
            #[doc =
                  " - __`data`__: A single sensor reading for that sensor listener"]
            #[doc = " - __`type`__: The sensor type for the data function"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_data_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                *mut ::cty::c_void,
                                                            arg3:
                                                                *mut ::cty::c_void,
                                                            arg4:
                                                                sensor_type_t)
                                           -> ::cty::c_int>;
            #[doc = " Callback for trigger compare functions."]
            #[doc = ""]
            #[doc = " - __`type`__: Type of sensor"]
            #[doc = " - __`low_thresh`__: The sensor low threshold"]
            #[doc = " - __`high_thresh`__: The sensor high threshold"]
            #[doc = " - __`arg`__: Ptr to data"]
            pub type sensor_trigger_cmp_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1:
                                                                sensor_type_t,
                                                            arg2:
                                                                *mut sensor_data_t,
                                                            arg3:
                                                                *mut sensor_data_t,
                                                            arg4:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            #[doc = " Callback for event notifications."]
            #[doc = ""]
            #[doc = " - __`sensor`__: The sensor that observed the event"]
            #[doc =
                  " - __`arg`__: The opaque argument provided during registration"]
            #[doc = " - __`event`__: The sensor event type that was observed"]
            pub type sensor_notifier_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                *mut ::cty::c_void,
                                                            arg3:
                                                                sensor_event_type_t)
                                           -> ::cty::c_int>;
            #[doc = " Callback for reporting a sensor read error."]
            #[doc = ""]
            #[doc = " - __`sensor`__: The sensor for which a read failed."]
            #[doc =
                  " - __`arg`__: The optional argument registered with the callback."]
            #[doc =
                  " - __`status`__: Indicates the cause of the read failure.  Determined by the"]
            #[doc = "               underlying sensor driver."]
            pub type sensor_error_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                *mut sensor,
                                                            arg:
                                                                *mut ::cty::c_void,
                                                            status:
                                                                ::cty::c_int)>;
            #[repr(C)]
            pub struct sensor_listener__bindgen_ty_1 {
                pub sle_next: *mut sensor_listener,
            }
            impl Default for sensor_listener__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct sensor_notifier__bindgen_ty_1 {
                pub sle_next: *mut sensor_notifier,
            }
            impl Default for sensor_notifier__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct sensor_type_traits__bindgen_ty_1 {
                pub sle_next: *mut sensor_type_traits,
            }
            impl Default for sensor_type_traits__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc =
                  " Read a single value from a sensor, given a specific sensor type"]
            #[doc = " (e.g. SENSOR_TYPE_PROXIMITY)."]
            #[doc = ""]
            #[doc = " - __`sensor`__: The sensor to read from"]
            #[doc =
                  " - __`type`__: The type(s) of sensor values to read.  Mask containing that type, provide"]
            #[doc = "        all, to get all values."]
            #[doc =
                  " - __`data_func`__: The function to call with each value read.  If NULL, it calls all"]
            #[doc = "        sensor listeners associated with this function."]
            #[doc =
                  " - __`arg`__: The argument to pass to the read callback."]
            #[doc =
                  " - __`timeout`__: Timeout. If block until result, specify OS_TIMEOUT_NEVER, 0 returns"]
            #[doc = "        immediately (no wait.)"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_read_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                sensor_type_t,
                                                            arg3:
                                                                sensor_data_func_t,
                                                            arg4:
                                                                *mut ::cty::c_void,
                                                            arg5: u32)
                                           -> ::cty::c_int>;
            #[doc =
                  " Get the configuration of the sensor for the sensor type.  This includes"]
            #[doc = " the value type of the sensor."]
            #[doc = ""]
            #[doc = " - __`sensor`__: Ptr to the sensor"]
            #[doc =
                  " - __`type`__: The type of sensor value to get configuration for"]
            #[doc =
                  " - __`cfg`__: A pointer to the sensor value to place the returned result into."]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_get_config_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                sensor_type_t,
                                                            arg3:
                                                                *mut sensor_cfg)
                                           -> ::cty::c_int>;
            #[doc = " Send a new configuration register set to the sensor."]
            #[doc = ""]
            #[doc = " - __`sensor`__: Ptr to the sensor-specific stucture"]
            #[doc =
                  " - __`arg`__: Ptr to the sensor-specific configuration structure"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_set_config_func_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                *mut ::cty::c_void)
                                           -> ::cty::c_int>;
            #[doc =
                  " Set the trigger and threshold values for a specific sensor for the sensor"]
            #[doc = " type."]
            #[doc = ""]
            #[doc = " - __`sensor`__: Ptr to the sensor"]
            #[doc = " - __`type`__: type of sensor"]
            #[doc = " - __`stt`__: Ptr to teh sensor traits"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_set_trigger_thresh_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                sensor_type_t,
                                                            stt:
                                                                *mut sensor_type_traits)
                                           -> ::cty::c_int>;
            #[doc =
                  " Clear the high/low threshold values for a specific sensor for the sensor"]
            #[doc = " type."]
            #[doc = ""]
            #[doc = " - __`sensor`__: Ptr to the sensor"]
            #[doc = " - __`type`__: Type of sensor"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_clear_trigger_thresh_t
                =
                ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                *mut sensor,
                                                            type_:
                                                                sensor_type_t)
                                           -> ::cty::c_int>;
            #[doc =
                  " Set the notification expectation for a targeted set of events for the"]
            #[doc =
                  " specific sensor. After this function returns successfully, the implementer"]
            #[doc =
                  " shall post corresponding event notifications to the sensor manager."]
            #[doc = ""]
            #[doc =
                  " - __`sensor`__: The sensor to expect notifications from."]
            #[doc =
                  " - __`event`__: The mask of event types to expect notifications from."]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_set_notification_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                sensor_event_type_t)
                                           -> ::cty::c_int>;
            #[doc =
                  " Unset the notification expectation for a targeted set of events for the"]
            #[doc = " specific sensor."]
            #[doc = ""]
            #[doc = " - __`sensor`__: The sensor."]
            #[doc = " - __`event`__: The mask of event types."]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_unset_notification_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor,
                                                            arg2:
                                                                sensor_event_type_t)
                                           -> ::cty::c_int>;
            #[doc = " Let driver handle interrupt in the sensor context"]
            #[doc = ""]
            #[doc = " - __`sensor`__: Ptr to the sensor"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero error code on failure."]
            pub type sensor_handle_interrupt_t
                =
                ::core::option::Option<unsafe extern "C" fn(sensor:
                                                                *mut sensor)
                                           -> ::cty::c_int>;
            #[doc = " Reset Sensor function Ptr"]
            #[doc = ""]
            #[doc = " - __`Ptr`__: to the sensor"]
            #[doc = ""]
            #[doc = " Return: 0 on success, non-zero on failure"]
            pub type sensor_reset_t
                =
                ::core::option::Option<unsafe extern "C" fn(arg1: *mut sensor)
                                           -> ::cty::c_int>;
            #[repr(C)]
            pub struct sensor__bindgen_ty_1 {
                pub slh_first: *mut sensor_listener,
            }
            impl Default for sensor__bindgen_ty_1 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct sensor__bindgen_ty_2 {
                pub slh_first: *mut sensor_notifier,
            }
            impl Default for sensor__bindgen_ty_2 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct sensor__bindgen_ty_3 {
                pub slh_first: *mut sensor_type_traits,
            }
            impl Default for sensor__bindgen_ty_3 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[repr(C)]
            pub struct sensor__bindgen_ty_4 {
                pub sle_next: *mut sensor,
            }
            impl Default for sensor__bindgen_ty_4 {
                fn default() -> Self { unsafe { ::core::mem::zeroed() } }
            }
            #[doc = "  Represents a single temperature sensor raw value"]
            #[repr(C, packed)]
            pub struct sensor_temp_raw_data {
                #[doc =
                      "  Raw temp from STM32 Internal Temp Sensor is 0 to 4095"]
                pub strd_temp_raw: u32,
                pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize], u8>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::default::Default for sensor_temp_raw_data {
                #[inline]
                fn default() -> sensor_temp_raw_data {
                    sensor_temp_raw_data{strd_temp_raw:
                                             ::core::default::Default::default(),
                                         _bitfield_1:
                                             ::core::default::Default::default(),}
                }
            }
            impl sensor_temp_raw_data {
                #[inline]
                pub fn strd_temp_raw_is_valid(&self) -> u8 {
                    unsafe {
                        ::core::mem::transmute(self._bitfield_1.get(0usize,
                                                                    1u8) as
                                                   u8)
                    }
                }
                #[inline]
                pub fn set_strd_temp_raw_is_valid(&mut self, val: u8) {
                    unsafe {
                        let val: u8 = ::core::mem::transmute(val);
                        self._bitfield_1.set(0usize, 1u8, val as u64)
                    }
                }
                #[inline]
                pub fn new_bitfield_1(strd_temp_raw_is_valid: u8)
                 -> __BindgenBitfieldUnit<[u8; 1usize], u8> {
                    let mut __bindgen_bitfield_unit:
                            __BindgenBitfieldUnit<[u8; 1usize], u8> =
                        Default::default();
                    __bindgen_bitfield_unit.set(0usize, 1u8,
                                                {
                                                    let strd_temp_raw_is_valid:
                                                            u8 =
                                                        unsafe {
                                                            ::core::mem::transmute(strd_temp_raw_is_valid)
                                                        };
                                                    strd_temp_raw_is_valid as
                                                        u64
                                                });
                    __bindgen_bitfield_unit
                }
            }
            extern "C" {
                #[doc =
                      "  Interpret `sensor_data` as a `sensor_temp_raw_data` struct that contains raw temp."]
                #[doc =
                      "  Copy the sensor data into `dest`.  Return 0 if successful."]
                pub fn get_temp_raw_data(sensor_data: *mut ::cty::c_void,
                                         dest: *mut sensor_temp_raw_data)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      "  Interpret `sensor_data` as a `sensor_temp_data` struct that contains computed temp."]
                #[doc =
                      "  Copy the sensor data into `dest`.  Return 0 if successful."]
                pub fn get_temp_data(sensor_data: *mut ::cty::c_void,
                                     dest: *mut sensor_temp_data)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc = "  Return the Mynewt device for the Mynewt sensor."]
                pub fn sensor_get_device(s: *mut sensor) -> *mut os_dev;
            }
            extern "C" {
                #[doc =
                      "  Return the name for the Mynewt device.  Assumes name is non-null."]
                pub fn device_get_name(device: *mut os_dev)
                 -> *const ::cty::c_char;
            }
            extern "C" {
                #[doc = "  Return the NULL sensor."]
                pub fn null_sensor() -> *mut sensor;
            }
            extern "C" {
                #[doc = "  Return non-zero if sensor is NULL."]
                pub fn is_null_sensor(p: *mut sensor) -> ::cty::c_int;
            }
            extern "C" {
                #[doc = "  Return non-zero if sensor data is NULL."]
                pub fn is_null_sensor_data(p: *mut ::cty::c_void)
                 -> ::cty::c_int;
            }
            extern "C" {
                #[doc =
                      "  Assume we are writing an object now.  Write the key name and start a child array."]
                #[doc = "  ```"]
                #[doc = "  {a:b --> {a:b, key:["]
                #[doc = "  ```"]
                pub fn json_helper_set_array(object: *mut ::cty::c_void,
                                             key: *const ::cty::c_char);
            }
            extern "C" {
                #[doc =
                      "  End the child array and resume writing the parent object."]
                #[doc = "  ```"]
                #[doc = "  {a:b, key:[... --> {a:b, key:[...]"]
                #[doc = "  ```"]
                pub fn json_helper_close_array(object: *mut ::cty::c_void,
                                               key: *const ::cty::c_char);
            }
            extern "C" {
                #[doc =
                      "  Assume we have called set_array.  Start an array item, assumed to be an object."]
                #[doc = "  ```"]
                #[doc = "  [... --> [...,"]
                #[doc = "  ```"]
                pub fn json_helper_object_array_start_item(key:
                                                               *const ::cty::c_char);
            }
            extern "C" {
                #[doc = "  End an array item, assumed to be an object."]
                #[doc = "  ```"]
                #[doc = "  [... --> [...,"]
                #[doc = "  ```"]
                pub fn json_helper_object_array_end_item(key:
                                                             *const ::cty::c_char);
            }
            extern "C" {
                #[doc =
                      "  Encode an int value into the current JSON encoding value `coap_json_value`"]
                pub fn json_helper_set_int(object: *mut ::cty::c_void,
                                           key: *const ::cty::c_char,
                                           value: u64);
            }
            extern "C" {
                #[doc =
                      "  Encode an unsigned int value into the current JSON encoding value `coap_json_value`"]
                pub fn json_helper_set_uint(object: *mut ::cty::c_void,
                                            key: *const ::cty::c_char,
                                            value: u64);
            }
            extern "C" {
                #[doc =
                      "  Encode a float value into the current JSON encoding value `coap_json_value`"]
                pub fn json_helper_set_float(object: *mut ::cty::c_void,
                                             key: *const ::cty::c_char,
                                             value: f32);
            }
            extern "C" {
                #[doc =
                      "  Encode a text value into the current JSON encoding value `coap_json_value`"]
                pub fn json_helper_set_text_string(object: *mut ::cty::c_void,
                                                   key: *const ::cty::c_char,
                                                   value:
                                                       *const ::cty::c_char);
            }
        }
    }
    /// TODO: Defined in repos/apache-mynewt-core/net/oic/src/api/oc_rep.c
    #[link(name = "net_oic")]
    extern "C" {
        /// Global CBOR encoder
        pub static mut g_encoder: encoding::tinycbor::CborEncoder;
        /// Global CBOR root map
        pub static mut root_map: encoding::tinycbor::CborEncoder;
    }
    /// Return type and error codes for Mynewt API
    pub mod result {
        use super::kernel::os;
        /// Common return type for Mynewt API.  If no error, returns `Ok(val)` where val has type T.
        /// Upon error, returns `Err(err)` where err is the MynewtError error code.
        pub type MynewtResult<T> = ::core::result::Result<T, MynewtError>;
        /// Error codes for Mynewt API
        #[repr(i32)]
        pub enum MynewtError {

            /// Error code 0 means no error.
            SYS_EOK = os::SYS_EOK as i32,
            SYS_ENOMEM = os::SYS_ENOMEM,
            SYS_EINVAL = os::SYS_EINVAL,
            SYS_ETIMEOUT = os::SYS_ETIMEOUT,
            SYS_ENOENT = os::SYS_ENOENT,
            SYS_EIO = os::SYS_EIO,
            SYS_EAGAIN = os::SYS_EAGAIN,
            SYS_EACCES = os::SYS_EACCES,
            SYS_EBUSY = os::SYS_EBUSY,
            SYS_ENODEV = os::SYS_ENODEV,
            SYS_ERANGE = os::SYS_ERANGE,
            SYS_EALREADY = os::SYS_EALREADY,
            SYS_ENOTSUP = os::SYS_ENOTSUP,
            SYS_EUNKNOWN = os::SYS_EUNKNOWN,
            SYS_EREMOTEIO = os::SYS_EREMOTEIO,
            SYS_EDONE = os::SYS_EDONE,
            SYS_EPERUSER = os::SYS_EPERUSER,
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::fmt::Debug for MynewtError {
            fn fmt(&self, f: &mut ::core::fmt::Formatter)
             -> ::core::fmt::Result {
                match (&*self,) {
                    (&MynewtError::SYS_EOK,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EOK");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ENOMEM,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ENOMEM");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EINVAL,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EINVAL");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ETIMEOUT,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ETIMEOUT");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ENOENT,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ENOENT");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EIO,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EIO");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EAGAIN,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EAGAIN");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EACCES,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EACCES");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EBUSY,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EBUSY");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ENODEV,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ENODEV");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ERANGE,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ERANGE");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EALREADY,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EALREADY");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_ENOTSUP,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_ENOTSUP");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EUNKNOWN,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EUNKNOWN");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EREMOTEIO,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EREMOTEIO");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EDONE,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EDONE");
                        debug_trait_builder.finish()
                    }
                    (&MynewtError::SYS_EPERUSER,) => {
                        let mut debug_trait_builder =
                            f.debug_tuple("SYS_EPERUSER");
                        debug_trait_builder.finish()
                    }
                }
            }
        }
        #[automatically_derived]
        #[allow(unused_qualifications)]
        impl ::core::cmp::PartialEq for MynewtError {
            #[inline]
            fn eq(&self, other: &MynewtError) -> bool {
                {
                    let __self_vi =
                        unsafe {
                            ::core::intrinsics::discriminant_value(&*self)
                        } as i32;
                    let __arg_1_vi =
                        unsafe {
                            ::core::intrinsics::discriminant_value(&*other)
                        } as i32;
                    if true && __self_vi == __arg_1_vi {
                        match (&*self, &*other) { _ => true, }
                    } else { false }
                }
            }
        }
        /// Cast `MynewtError` to `i32`
        impl From<MynewtError> for i32 {
            /// Cast `MynewtError` to `i32`
            fn from(err: MynewtError) -> Self { err as i32 }
        }
        /// Cast `i32` to `MynewtError`
        impl From<i32> for MynewtError {
            /// Cast `i32` to `MynewtError`
            fn from(num: i32) -> Self {
                unsafe { ::core::mem::transmute::<i32, MynewtError>(num) }
            }
        }
    }
}
#[allow(dead_code)]
mod base {
    //!  Common declarations for the application.  Includes custom sensor declarations.
    use cty::c_char;
    use crate::mynewt::kernel::os::os_dev;
    use crate::mynewt::hw::sensor::{self, sensor_ptr, sensor_data_ptr,
                                    sensor_temp_data, sensor_type_t};
    ///  Display message `msg` on the Arm Semihosting console (via OpenOCD).
    pub fn console_print(msg: &[u8]) {
        let len = msg.len();
        unsafe { console_buffer(msg.as_ptr(), len as u32); console_flush(); }
    }
    ///  Import the custom interop helper library at `libs/mynewt_rust`
    #[link(name = "libs_mynewt_rust")]
    extern "C" {
        ///  Initialise the Mynewt system.  Start the Mynewt drivers and libraries.  Equivalent to `sysinit()` macro in C.
        ///  C API: `void rust_sysinit()`
        pub fn rust_sysinit();
        ///  Interpret `sensor_data` as a `sensor_temp_raw_data` struct that contains raw temp.
        ///  Copy the sensor data into `dest`.  Return 0 if successful.
        ///  C API: `int get_temp_raw_data(void *sensor_data, struct sensor_temp_raw_data *dest)`
        pub fn get_temp_raw_data(sensor_data: sensor_data_ptr,
                                 dest: *mut sensor_temp_raw_data) -> i32;
        ///  Interpret `sensor_data` as a `sensor_temp_data` struct that contains computed temp.
        ///  Copy the sensor data into `dest`.  Return 0 if successful.
        ///  C API: `int get_temp_data(void *sensor_data, struct sensor_temp_data *dest)`
        pub fn get_temp_data(sensor_data: sensor_data_ptr,
                             dest: *mut sensor_temp_data) -> i32;
        ///  Return the Mynewt device for the Mynewt sensor.
        ///  C API: `struct os_dev *sensor_get_device(struct sensor *s)`
        pub fn sensor_get_device(sensor: sensor_ptr) -> *mut os_dev;
        ///  Return the name for the Mynewt device.  Assumes name is non-null.
        ///  C API: `const char *device_get_name(struct os_dev *device)`
        pub fn device_get_name(device: *mut os_dev) -> *const c_char;
        ///  Return the NULL sensor.
        ///  C API: `struct sensor *null_sensor(void)`
        pub fn null_sensor() -> sensor_ptr;
        ///  Return non-zero if sensor is NULL.
        ///  C API: `int is_null_sensor(struct sensor *p)`
        pub fn is_null_sensor(sensor: sensor_ptr) -> bool;
        ///  Return non-zero if sensor data is NULL.
        ///  C API: `int is_null_sensor_data(void *p)`
        pub fn is_null_sensor_data(sensor_data: sensor_data_ptr) -> bool;
    }
    ///  Import the custom Mynewt library for displaying messages on the Arm Semihosting Console (via OpenOCD).
    ///  The library is located at `libs/semihosting_console`
    #[link(name = "libs_semihosting_console")]
    extern "C" {
        ///  Add the string to the output buffer.
        ///  C API: `void console_buffer(const char *buffer, unsigned int length)`
        pub fn console_buffer(buffer: *const u8, length: u32);
        ///  Write a byte in hexadecimal to the output buffer.
        ///  C API: `void console_printhex(uint8_t v)`
        pub fn console_printhex(v: u8);
        ///  Write a float to the output buffer, with 1 decimal place.
        ///  C API: `void console_printfloat(float f)`
        pub fn console_printfloat(f: f32);
        ///  Append "length" number of bytes from "buffer" to the output buffer in hex format.
        ///  C API: `void console_dump(const uint8_t *buffer, unsigned int len)`
        pub fn console_dump(buffer: *const u8, len: u32);
        ///  Flush the output buffer to the console.
        ///  C API: `void console_flush(void)`
        pub fn console_flush();
    }
    ///  We will open internal temperature sensor `temp_stm32_0`.
    ///  Must sync with apps/my_sensor_app/src/listen_sensor.h
    pub const SENSOR_DEVICE: *const c_char = TEMP_STM32_DEVICE;
    pub const TEMP_STM32_DEVICE: *const c_char =
        b"temp_stm32_0\0".as_ptr() as *const c_char;
    ///  Set to raw sensor type
    pub const TEMP_SENSOR_TYPE: sensor_type_t =
        SENSOR_TYPE_AMBIENT_TEMPERATURE_RAW;
    ///  Return integer sensor values
    pub const TEMP_SENSOR_VALUE_TYPE: i32 =
        sensor::SENSOR_VALUE_TYPE_INT32 as i32;
    ///  Use key (field name) `t` to transmit raw temperature to CoAP Server or Collector Node
    pub const TEMP_SENSOR_KEY: &str = "t";
    ///  Sensor type for raw temperature sensor.
    ///  Must sync with libs/custom_sensor/include/custom_sensor/custom_sensor.h
    pub const SENSOR_TYPE_AMBIENT_TEMPERATURE_RAW: sensor_type_t =
        sensor::sensor_type_t_SENSOR_TYPE_USER_DEFINED_1;
    ///  Represents a decoded sensor data value. Since temperature may be integer (raw)
    ///  or float (computed), we use the struct to return both integer and float values.
    pub struct SensorValue {
        ///  Null-terminated string for the key.  `t` for raw temp, `tmp` for computed. When transmitted to CoAP Server or Collector Node, the key (field name) to be used.
        pub key: &'static str,
        ///  The type of the sensor value and the value.
        pub val: SensorValueType,
    }
    ///  Default sensor value is `None`
    impl Default for SensorValue {
        #[inline]
        fn default() -> SensorValue {
            SensorValue{key: "", val: SensorValueType::None,}
        }
    }
    ///  Represents the type and value of a sensor data value.
    pub enum SensorValueType {

        ///  No value.
        None,

        ///  32-bit unsigned integer. For raw temp, contains the raw temp integer value
        Uint(u32),

        ///  32-bit float. For computed temp, contains the computed temp float value
        Float(f32),
    }
    ///  Represents a single temperature sensor raw value.
    ///  Must sync with libs/custom_sensor/include/custom_sensor/custom_sensor.h
    #[repr(C, packed)]
    pub struct sensor_temp_raw_data {
        ///  Raw temp from STM32 Internal Temp Sensor is 0 to 4095.
        pub strd_temp_raw: u32,
        ///  1 if data is valid
        pub strd_temp_raw_is_valid: u8,
    }
}
mod listen_sensor {
    //!  Poll the temperature sensor every 10 seconds.  We support 2 types of temperature sensors:
    //!  (1)  BME280 Temperature Sensor, connected to Blue Pill on port SPI1.
    //!       This sensor is selected if BME280_OFB is defined in syscfg.yml.
    //!  (2)  Blue Pill internal temperature sensor, connected to port ADC1 on channel 16
    //!       This sensor is selected if TEMP_STM32 is defined in syscfg.yml.
    //!  If this is the Collector Node, send the sensor data to the CoAP Server after polling.
    //!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust/apps/my_sensor_app/OLDsrc/listen_sensor.c`
    use cstr_core::CStr;
    use cty::c_char;
    use crate::fill_zero;
    use crate::base::*;
    use crate::send_coap::send_sensor_data;
    use crate::mynewt::{result::*,
                        hw::sensor::{self, sensor_ptr, sensor_arg,
                                     sensor_data_ptr, sensor_listener,
                                     sensor_temp_data, sensor_type_t}};
    ///  Poll every 10,000 milliseconds (10 seconds)  
    const SENSOR_POLL_TIME: u32 = (10 * 1000);
    ///  Indicate that this is a listener callback
    const LISTENER_CB: sensor_arg = 1 as sensor_arg;
    ///  For Sensor Node and Standalone Node: Start polling the temperature sensor 
    ///  every 10 seconds in the background.  After polling the sensor, call the 
    ///  Listener Function to send the sensor data to the Collector Node (if this is a Sensor Node)
    ///  or CoAP Server (is this is a Standalone Node).
    ///  For Collector Node: Start the Listeners for Remote Sensor 
    ///  Otherwise this is a Standalone Node with ESP8266, or a Sensor Node with nRF24L01.
    ///  Return `Ok()` if successful, else return `Err()` with `MynewtError` error code inside.
    pub fn start_sensor_listener() -> MynewtResult<()> {
        console_print(b"TMP poll \n");
        let rc =
            unsafe {
                sensor::sensor_set_poll_rate_ms(SENSOR_DEVICE,
                                                SENSOR_POLL_TIME)
            };
        {
            match (&rc, &0) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                          "`,\n right: `",
                                                                                          "`"],
                                                                                        &match (&&*left_val,
                                                                                                &&*right_val)
                                                                                             {
                                                                                             (arg0,
                                                                                              arg1)
                                                                                             =>
                                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                           ::core::fmt::Debug::fmt),
                                                                                              ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                           ::core::fmt::Debug::fmt)],
                                                                                         }),
                                                         &("src/listen_sensor.rs",
                                                           47u32, 5u32))
                        }
                    }
                }
            }
        };
        let sensor =
            unsafe {
                sensor::sensor_mgr_find_next_bydevname(SENSOR_DEVICE,
                                                       null_sensor())
            };
        if !unsafe { !is_null_sensor(sensor) } {
            {
                ::core::panicking::panic(&("assertion failed: unsafe { !is_null_sensor(sensor) }",
                                           "src/listen_sensor.rs", 51u32,
                                           5u32))
            }
        };
        let listener =
            sensor_listener{sl_sensor_type: TEMP_SENSOR_TYPE,
                            sl_func: sensor::as_untyped(read_temperature),
                            sl_arg:
                                LISTENER_CB,
                                               ..unsafe {
                                                     ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_listener>()],
                                                                              sensor_listener>([0;
                                                                                                   ::core::mem::size_of::<sensor_listener>()])
                                                 }};
        sensor::register_listener(sensor, listener)?;
        Ok(())
    }
    ///  This listener function is called by Mynewt every 10 seconds (for local sensors) or when sensor data is received
    ///  (for Remote Sensors).  Mynewt has fetched the raw or computed temperature value, passed through `sensor_data`.
    ///  If this is a Sensor Node, we send the sensor data to the Collector Node.
    ///  If this is a Collector Node or Standalone Node, we send the sensor data to the CoAP server.  
    ///  Return 0 if we have processed the sensor data successfully.
    extern "C" fn read_temperature(sensor: sensor_ptr, _arg: sensor_arg,
                                   sensor_data: sensor_data_ptr,
                                   sensor_type: sensor_type_t)
     -> MynewtError {
        console_print(b"read_temperature\n");
        if unsafe { is_null_sensor_data(sensor_data) } {
            return MynewtError::SYS_EINVAL;
        }
        if !unsafe { !is_null_sensor(sensor) } {
            {
                ::core::panicking::panic(&("assertion failed: unsafe { !is_null_sensor(sensor) }",
                                           "src/listen_sensor.rs", 87u32,
                                           5u32))
            }
        };
        let device = unsafe { sensor_get_device(sensor) };
        let device_name_ptr: *const c_char =
            unsafe { device_get_name(device) };
        let device_name: &CStr = unsafe { CStr::from_ptr(device_name_ptr) };
        let temp_sensor_value = get_temperature(sensor_data, sensor_type);
        if let SensorValueType::None = temp_sensor_value.val {
            if !false {
                {
                    ::core::panicking::panic(&("assertion failed: false",
                                               "src/listen_sensor.rs", 99u32,
                                               60u32))
                }
            };
        }
        let rc = send_sensor_data(&temp_sensor_value, device_name);
        if let Err(err) = rc {
            if err == MynewtError::SYS_EAGAIN {
                console_print(b"TMP network not ready\n");
                return MynewtError::SYS_EOK;
            }
        }
        MynewtError::SYS_EOK
    }
    ///  Get the temperature value, raw or computed.  `sensor_data` contains the raw or computed temperature. 
    ///  `sensor_type` indicates whether `sensor_data` contains raw or computed temperature.  We return 
    ///  the raw or computed temperature, as well as the key and value type.
    #[allow(unreachable_patterns)]
    #[allow(unused_variables)]
    fn get_temperature(sensor_data: sensor_data_ptr,
                       sensor_type: sensor_type_t) -> SensorValue {
        let mut return_value = SensorValue::default();
        match sensor_type {
            SENSOR_TYPE_AMBIENT_TEMPERATURE_RAW => {
                let mut rawtempdata =
                    unsafe {
                        ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_temp_raw_data>()],
                                                 sensor_temp_raw_data>([0;
                                                                           ::core::mem::size_of::<sensor_temp_raw_data>()])
                    };
                let rc =
                    unsafe {
                        get_temp_raw_data(sensor_data, &mut rawtempdata)
                    };
                if !(rc == 0) {
                    {
                        ::core::panicking::panic(&("assertion failed: rc == 0",
                                                   "src/listen_sensor.rs",
                                                   131u32, 13u32))
                    }
                };
                if rawtempdata.strd_temp_raw_is_valid == 0 {
                    return return_value;
                }
                return_value.val =
                    SensorValueType::Uint(rawtempdata.strd_temp_raw);
                console_print(b"TMP listener got rawtmp \n");
            }
            SENSOR_TYPE_AMBIENT_TEMPERATURE => {
                let mut tempdata =
                    unsafe {
                        ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_temp_data>()],
                                                 sensor_temp_data>([0;
                                                                       ::core::mem::size_of::<sensor_temp_data>()])
                    };
                let rc = unsafe { get_temp_data(sensor_data, &mut tempdata) };
                if !(rc == 0) {
                    {
                        ::core::panicking::panic(&("assertion failed: rc == 0",
                                                   "src/listen_sensor.rs",
                                                   144u32, 13u32))
                    }
                };
                if tempdata.std_temp_is_valid() == 0 { return return_value; }
                return_value.val = SensorValueType::Float(tempdata.std_temp);
            }
            _ => {
                if !false {
                    {
                        ::core::panicking::panic(&("assertion failed: false",
                                                   "src/listen_sensor.rs",
                                                   158u32, 13u32))
                    }
                };
                return return_value;
            }
        }
        return_value.key = TEMP_SENSOR_KEY;
        return_value
    }
}
mod send_coap {
    //!  Send sensor data to a CoAP Server or a Collector Node.  The CoAP payload will be encoded as JSON
    //!  for CoAP Server and CBOR for Collector Node.  The sensor data will be transmitted to 
    //!  CoAP Server over WiFi via the ESP8266 transceiver, and to Collector Node via nRF24L01 transceiver.
    //!  This enables transmission of Sensor Data to a local Sensor Network (via nRF24L01)
    //!  and to the internet (via ESP8266).  For sending to Collector Node we use raw temperature (integer) 
    //!  instead of computed temperature (floating-point) to make the encoding simpler and faster.
    //!  Note that we are using a patched version of apps/my_sensor_app/src/vsscanf.c that
    //!  fixes ESP8266 response parsing bugs.  The patched file must be present in that location.
    //!  This is the Rust version of `https://github.com/lupyuen/stm32bluepill-mynewt-sensor/blob/rust/apps/my_sensor_app/OLDsrc/send_coap.c`
    use cstr_core::CStr;
    use cty::*;
    use crate::{coap, d, fill_zero};
    use crate::base::*;
    use crate::mynewt::{result::*,
                        kernel::os::{self, os_task, os_stack_t,
                                     os_task_func_t, os_time_t},
                        encoding::{json_context::{self, JSON_CONTEXT,
                                                  ToBytesOptionalNull},
                                   tinycbor},
                        libs::{mynewt_rust, sensor_network,
                               sensor_coap::{self, sensor_value}}};
    /// Represents a null-terminated byte string, suitable for passing to Mynewt APIs as `* const char`
    pub struct StrN {
        /// Byte string terminated with null
        bytestr: &'static [u8],
    }
    impl StrN {
        /// Create a new byte string:
        /// ```
        /// StrN::new(b"network\0")
        /// strn!("network")
        /// ```
        pub fn new(bs: &'static [u8]) -> StrN {
            {
                match (&bs.last(), &Some(0)) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`"],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val)
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt)],
                                                                                             }),
                                                             &("src/send_coap.rs",
                                                               59u32, 9u32))
                            }
                        }
                    }
                }
            };
            let res = StrN{bytestr: bs,};
            res
        }
    }
    fn test_safe_wrap() {
        "-------------------------------------------------------------";
        "-------------------------------------------------------------";
        type Ptr = *mut ::cty::c_void;
        const NULL: Ptr = 0 as Ptr;
        task_init(&mut NETWORK_TASK, (/*ERROR*/), Some(network_task_func),
                  NULL, 10, os::OS_WAIT_FOREVER as u32,
                  NETWORK_TASK_STACK.as_ptr() as *mut os_stack_t,
                  NETWORK_TASK_STACK_SIZE as u16);
        pub fn task_init(arg1: *mut os_task, arg2: &StrN,
                         arg3: os_task_func_t, arg4: Ptr, arg5: u8,
                         arg6: os_time_t, arg7: *mut os_stack_t, arg8: u16)
         -> MynewtResult<()> {
            extern "C" {
                pub fn os_task_init(arg1: *mut os_task,
                                    arg2: *const ::cty::c_char,
                                    arg3: os_task_func_t,
                                    arg4: *mut ::cty::c_void, arg5: u8,
                                    arg6: os_time_t, arg7: *mut os_stack_t,
                                    arg8: u16) -> ::cty::c_int;
            }
            unsafe {
                let res =
                    os_task_init(arg1, arg2, arg3, arg4, arg5, arg6, arg7,
                                 arg8);
                if res == 0 { Ok(()) } else { Err(res) }
            }
        }
        #[doc = " Initialize a task."]
        #[doc = ""]
        #[doc =
              " This function initializes the task structure pointed to by t,"]
        #[doc =
              " clearing and setting it's stack pointer, provides sane defaults"]
        #[doc =
              " and sets the task as ready to run, and inserts it into the operating"]
        #[doc = " system scheduler."]
        #[doc = ""]
        #[doc = " - __`t`__: The task to initialize"]
        #[doc = " - __`name`__: The name of the task to initialize"]
        #[doc = " - __`func`__: The task function to call"]
        #[doc = " - __`arg`__: The argument to pass to this task function"]
        #[doc = " - __`prio`__: The priority at which to run this task"]
        #[doc =
              " - __`sanity_itvl`__: The time at which this task should check in with the"]
        #[doc =
              "                    sanity task.  OS_WAIT_FOREVER means never check in"]
        #[doc = "                    here."]
        #[doc =
              " - __`stack_bottom`__: A pointer to the bottom of a task's stack"]
        #[doc = " - __`stack_size`__: The overall size of the task's stack."]
        #[doc = ""]
        #[doc = " Return: 0 on success, non-zero on failure."]
        fn dummy() { }
    }
    ///  Storage for Network Task: Mynewt task object will be saved here.
    static mut NETWORK_TASK: os_task =
        unsafe {
            ::core::mem::transmute::<[u8; ::core::mem::size_of::<os_task>()],
                                     os_task>([0;
                                                  ::core::mem::size_of::<os_task>()])
        };
    ///  Stack space for Network Task, initialised to 0.
    static mut NETWORK_TASK_STACK: [os_stack_t; NETWORK_TASK_STACK_SIZE] =
        [0; NETWORK_TASK_STACK_SIZE];
    ///  Size of the stack (in 4-byte units). Previously `OS_STACK_ALIGN(256)`  
    const NETWORK_TASK_STACK_SIZE: usize = 256;
    ///  Set to true when network tasks have been completed
    static mut NETWORK_IS_READY: bool = false;
    ///  Start the Network Task in the background.  The Network Task prepares the network drivers
    ///  (ESP8266 and nRF24L01) for transmitting sensor data messages.  
    ///  Connecting the ESP8266 to the WiFi access point may be slow so we do this in the background.
    ///  Also perform WiFi Geolocation if it is enabled.  Return 0 if successful.
    pub fn start_network_task() -> MynewtResult<()> {
        console_print(b"NET start\n");
        let rc =
            unsafe {
                os::os_task_init(&mut NETWORK_TASK,
                                 b"network\0".as_ptr() as *const c_char,
                                 Some(network_task_func),
                                 0 as *mut ::cty::c_void, 10,
                                 os::OS_WAIT_FOREVER as u32,
                                 NETWORK_TASK_STACK.as_ptr() as
                                     *mut os_stack_t,
                                 NETWORK_TASK_STACK_SIZE as u16)
            };
        {
            match (&rc, &0) {
                (left_val, right_val) => {
                    if !(*left_val == *right_val) {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                          "`,\n right: `",
                                                                                          "`"],
                                                                                        &match (&&*left_val,
                                                                                                &&*right_val)
                                                                                             {
                                                                                             (arg0,
                                                                                              arg1)
                                                                                             =>
                                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                           ::core::fmt::Debug::fmt),
                                                                                              ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                           ::core::fmt::Debug::fmt)],
                                                                                         }),
                                                         &("src/send_coap.rs",
                                                           208u32, 5u32))
                        }
                    }
                }
            }
        };
        Ok(())
    }
    ///  Network Task runs this function in the background to prepare the network drivers
    ///  (ESP8266 and nRF24L01) for transmitting sensor data messages.  Also perform WiFi Geolocation if it is enabled.
    ///  For Collector Node and Standalone Node: We connect the ESP8266 to the WiFi access point. 
    ///  Connecting the ESP8266 to the WiFi access point may be slow so we do this in the background.
    ///  Register the ESP8266 driver as the network transport for CoAP Server.  
    ///  For Collector Node and Sensor Nodes: We register the nRF24L01 driver as the network transport for 
    ///  CoAP Collector.
    extern "C" fn network_task_func(_arg: *mut ::cty::c_void) {
        console_print(b"NET start\n");
        if !unsafe { !NETWORK_IS_READY } {
            {
                ::core::panicking::panic(&("assertion failed: unsafe { !NETWORK_IS_READY }",
                                           "src/send_coap.rs", 220u32, 37u32))
            }
        };
        if unsafe {
               sensor_network::is_standalone_node() ||
                   sensor_network::is_collector_node()
           } {
            let rc = unsafe { sensor_network::register_server_transport() };
            {
                match (&rc, &0) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`"],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val)
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt)],
                                                                                             }),
                                                             &("src/send_coap.rs",
                                                               228u32, 75u32))
                            }
                        }
                    }
                }
            };
        }
        if unsafe {
               sensor_network::is_collector_node() ||
                   sensor_network::is_sensor_node()
           } {
            let rc =
                unsafe { sensor_network::register_collector_transport() };
            {
                match (&rc, &0) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            {
                                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left == right)`\n  left: `",
                                                                                              "`,\n right: `",
                                                                                              "`"],
                                                                                            &match (&&*left_val,
                                                                                                    &&*right_val)
                                                                                                 {
                                                                                                 (arg0,
                                                                                                  arg1)
                                                                                                 =>
                                                                                                 [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                               ::core::fmt::Debug::fmt),
                                                                                                  ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                               ::core::fmt::Debug::fmt)],
                                                                                             }),
                                                             &("src/send_coap.rs",
                                                               236u32, 78u32))
                            }
                        }
                    }
                }
            };
        }
        unsafe { NETWORK_IS_READY = true; }
        loop  {
            console_print(b"NET free mbuf %d\n");
            unsafe { os::os_time_delay(10 * os::OS_TICKS_PER_SEC); }
        }
    }
    ///  Compose a CoAP message (CBOR or JSON) with the sensor value in `val` and transmit to the
    ///  Collector Node (if this is a Sensor Node) or to the CoAP Server (if this is a Collector Node
    ///  or Standalone Node).  
    ///  For Sensor Node or Standalone Node: sensor_node is the sensor name (`bme280_0` or `temp_stm32_0`)
    ///  For Collector Node: sensor_node is the Sensor Node Address of the Sensor Node that transmitted
    ///  the sensor data (like `b3b4b5b6f1`)
    ///  The message will be enqueued for transmission by the CoAP / OIC Background Task 
    ///  so this function will return without waiting for the message to be transmitted.  
    ///  Return 0 if successful, SYS_EAGAIN if network is not ready yet.
    pub fn send_sensor_data(sensor_val: &SensorValue, sensor_node: &CStr)
     -> MynewtResult<()> {
        console_print(b"send_sensor_data\n");
        let mut val =
            unsafe {
                ::core::mem::transmute::<[u8; ::core::mem::size_of::<sensor_value>()],
                                         sensor_value>([0;
                                                           ::core::mem::size_of::<sensor_value>()])
            };
        if unsafe {
               sensor_network::should_send_to_collector(&mut val,
                                                        sensor_node.as_ptr())
           } {
            return send_sensor_data_to_collector(sensor_val, sensor_node);
        }
        send_sensor_data_to_server(sensor_val, sensor_node)
    }
    ///  Compose a CoAP JSON message with the Sensor Key (field name) and Value in val 
    ///  and send to the CoAP server and URI.  The Sensor Value may be integer or float.
    ///  For temperature, the Sensor Key is either `t` for raw temperature (integer, from 0 to 4095) 
    ///  or `tmp` for computed temperature (float).
    ///  The message will be enqueued for transmission by the CoAP / OIC 
    ///  Background Task so this function will return without waiting for the message 
    ///  to be transmitted.  Return 0 if successful, `SYS_EAGAIN` if network is not ready yet.
    ///  For the CoAP server hosted at thethings.io, the CoAP payload should be encoded in JSON like this:
    ///  ```
    ///  {"values":[
    ///    {"key":"device", "value":"0102030405060708090a0b0c0d0e0f10"},
    ///    {"key":"tmp",    "value":28.7},
    ///    {"key":"...",    "value":... },
    ///    ... ]}
    ///  ```
    fn send_sensor_data_to_server(sensor_val: &SensorValue, node_id: &CStr)
     -> MynewtResult<()> {
        if let SensorValueType::None = sensor_val.val {
            if !false {
                {
                    ::core::panicking::panic(&("assertion failed: false",
                                               "src/send_coap.rs", 290u32,
                                               53u32))
                }
            };
        }
        {
            match (&node_id.to_bytes()[0], &0) {
                (left_val, right_val) => {
                    if *left_val == *right_val {
                        {
                            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["assertion failed: `(left != right)`\n  left: `",
                                                                                          "`,\n right: `",
                                                                                          "`"],
                                                                                        &match (&&*left_val,
                                                                                                &&*right_val)
                                                                                             {
                                                                                             (arg0,
                                                                                              arg1)
                                                                                             =>
                                                                                             [::core::fmt::ArgumentV1::new(arg0,
                                                                                                                           ::core::fmt::Debug::fmt),
                                                                                              ::core::fmt::ArgumentV1::new(arg1,
                                                                                                                           ::core::fmt::Debug::fmt)],
                                                                                         }),
                                                         &("src/send_coap.rs",
                                                           292u32, 5u32))
                        }
                    }
                }
            }
        };
        if unsafe { !NETWORK_IS_READY } {
            return Err(MynewtError::SYS_EAGAIN);
        }
        let device_id_ptr = unsafe { sensor_network::get_device_id() };
        let device_id: &CStr = unsafe { CStr::from_ptr(device_id_ptr) };
        let rc =
            unsafe { sensor_network::init_server_post(0 as *const c_char) };
        if !rc {
            {
                ::core::panicking::panic(&("assertion failed: rc",
                                           "src/send_coap.rs", 300u32, 80u32))
            }
        };
        let _payload =
            {
                "begin json root";
                {
                    "begin json coap_root";
                    unsafe { sensor_coap::json_rep_start_root_object() }
                    {
                        {
                            "begin json coap_array , object : JSON_CONTEXT , key : values";
                            {
                                "<< jarri , o: JSON_CONTEXT, k: values";
                                let key_with_null: &str = "values\u{0}";
                                unsafe {
                                    mynewt_rust::json_helper_set_array(JSON_CONTEXT.to_void_ptr(),
                                                                       JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()));
                                };
                            };
                            {
                                " >>  >> \"device\" >> : device_id , \"node\" : node_id , sensor_val ,";
                                "add1 key : \"device\" value : $crate::parse!(@ json device_id) to object :\nJSON_CONTEXT";
                                {
                                    "begin json coap_item_str , parent : JSON_CONTEXT , key : \"device\" , val :\n$crate::parse!(@ json device_id)";
                                    {
                                        "begin json coap_item , array : JSON_CONTEXT";
                                        {
                                            "<< jitmi c: JSON_CONTEXT";
                                            let key_with_null: &str =
                                                "JSON_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt_rust::json_helper_object_array_start_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        {
                                            {
                                                "-- jtxti o: JSON_CONTEXT, k: key, v: \"device\"";
                                                let key_with_null: &str =
                                                    "key\u{0}";
                                                let value_with_opt_null:
                                                        &[u8] =
                                                    "device".to_bytes_optional_nul();
                                                unsafe {
                                                    mynewt_rust::json_helper_set_text_string(JSON_CONTEXT.to_void_ptr(),
                                                                                             JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()),
                                                                                             JSON_CONTEXT.value_to_cstr(value_with_opt_null))
                                                };
                                            };
                                            {
                                                "-- jtxti o: JSON_CONTEXT, k: value, v: $crate::parse!(@ json device_id)";
                                                let key_with_null: &str =
                                                    "value\u{0}";
                                                let value_with_opt_null:
                                                        &[u8] =
                                                    device_id.to_bytes_optional_nul();
                                                unsafe {
                                                    mynewt_rust::json_helper_set_text_string(JSON_CONTEXT.to_void_ptr(),
                                                                                             JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()),
                                                                                             JSON_CONTEXT.value_to_cstr(value_with_opt_null))
                                                };
                                            };
                                        };
                                        {
                                            ">>";
                                            let key_with_null: &str =
                                                "JSON_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt_rust::json_helper_object_array_end_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        "end json coap_item";
                                    };
                                    "end json coap_item_str";
                                };
                                "--------------------";
                                " >>  >> \"node\" >> : node_id , sensor_val ,";
                                "add1 key : \"node\" value : $crate::parse!(@ json node_id) to object :\nJSON_CONTEXT";
                                {
                                    "begin json coap_item_str , parent : JSON_CONTEXT , key : \"node\" , val :\n$crate::parse!(@ json node_id)";
                                    {
                                        "begin json coap_item , array : JSON_CONTEXT";
                                        {
                                            "<< jitmi c: JSON_CONTEXT";
                                            let key_with_null: &str =
                                                "JSON_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt_rust::json_helper_object_array_start_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        {
                                            {
                                                "-- jtxti o: JSON_CONTEXT, k: key, v: \"node\"";
                                                let key_with_null: &str =
                                                    "key\u{0}";
                                                let value_with_opt_null:
                                                        &[u8] =
                                                    "node".to_bytes_optional_nul();
                                                unsafe {
                                                    mynewt_rust::json_helper_set_text_string(JSON_CONTEXT.to_void_ptr(),
                                                                                             JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()),
                                                                                             JSON_CONTEXT.value_to_cstr(value_with_opt_null))
                                                };
                                            };
                                            {
                                                "-- jtxti o: JSON_CONTEXT, k: value, v: $crate::parse!(@ json node_id)";
                                                let key_with_null: &str =
                                                    "value\u{0}";
                                                let value_with_opt_null:
                                                        &[u8] =
                                                    node_id.to_bytes_optional_nul();
                                                unsafe {
                                                    mynewt_rust::json_helper_set_text_string(JSON_CONTEXT.to_void_ptr(),
                                                                                             JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()),
                                                                                             JSON_CONTEXT.value_to_cstr(value_with_opt_null))
                                                };
                                            };
                                        };
                                        {
                                            ">>";
                                            let key_with_null: &str =
                                                "JSON_CONTEXT\u{0}";
                                            unsafe {
                                                mynewt_rust::json_helper_object_array_end_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                            };
                                        };
                                        "end json coap_item";
                                    };
                                    "end json coap_item_str";
                                };
                                "--------------------";
                                " >>  >> sensor_val >> ,";
                                "--------------------";
                                {
                                    "begin json coap_item_int_val , c : JSON_CONTEXT , val : sensor_val";
                                    if let SensorValueType::Uint(val) =
                                           sensor_val.val {
                                        {
                                            "begin json coap_item_int , key : sensor_val.key , value : val";
                                            {
                                                "begin json coap_item , array : JSON_CONTEXT";
                                                {
                                                    "<< jitmi c: JSON_CONTEXT";
                                                    let key_with_null: &str =
                                                        "JSON_CONTEXT\u{0}";
                                                    unsafe {
                                                        mynewt_rust::json_helper_object_array_start_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                    };
                                                };
                                                {
                                                    {
                                                        "-- jtxte o: JSON_CONTEXT, k: \"key\", v: sensor_val.key";
                                                        let key_with_opt_null:
                                                                &[u8] =
                                                            "key".to_bytes_optional_nul();
                                                        let value_with_opt_null:
                                                                &[u8] =
                                                            sensor_val.key.to_bytes_optional_nul();
                                                        unsafe {
                                                            mynewt_rust::json_helper_set_text_string(JSON_CONTEXT.to_void_ptr(),
                                                                                                     JSON_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                                     JSON_CONTEXT.value_to_cstr(value_with_opt_null))
                                                        };
                                                    };
                                                    {
                                                        "-- jinte o: JSON_CONTEXT, k: \"value\", v: val";
                                                        let key_with_opt_null:
                                                                &[u8] =
                                                            "value".to_bytes_optional_nul();
                                                        let value =
                                                            val as u64;
                                                        unsafe {
                                                            mynewt_rust::json_helper_set_int(JSON_CONTEXT.to_void_ptr(),
                                                                                             JSON_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                                             value)
                                                        };
                                                    };
                                                };
                                                {
                                                    ">>";
                                                    let key_with_null: &str =
                                                        "JSON_CONTEXT\u{0}";
                                                    unsafe {
                                                        mynewt_rust::json_helper_object_array_end_item(JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                                    };
                                                };
                                                "end json coap_item";
                                            };
                                            "end json coap_item_int";
                                        };
                                    } else {
                                        unsafe {
                                            JSON_CONTEXT.fail(json_context::JsonError::VALUE_NOT_UINT)
                                        };
                                    }
                                    "end json coap_item_int_val";
                                };
                                "--------------------";
                            };
                            {
                                ">>";
                                let key_with_null: &str = "values\u{0}";
                                unsafe {
                                    mynewt_rust::json_helper_close_array(JSON_CONTEXT.to_void_ptr(),
                                                                         JSON_CONTEXT.key_to_cstr(key_with_null.as_bytes()))
                                };
                            };
                            "end json coap_array";
                        };
                    };
                    unsafe { sensor_coap::json_rep_end_root_object() }
                    "end json coap_root";
                };
                "end json root";
                ()
            };
        let rc = unsafe { sensor_network::do_server_post() };
        if !rc {
            {
                ::core::panicking::panic(&("assertion failed: rc",
                                           "src/send_coap.rs", 319u32, 60u32))
            }
        };
        console_print(b"NET view your sensor at \nhttps://blue-pill-geolocate.appspot.com?device=%s\n");
        Ok(())
    }
    ///  Compose a CoAP CBOR message with the Sensor Key (field name) and Value in val and 
    ///  transmit to the Collector Node.  The Sensor Value should be integer not float since
    ///  we transmit integers only to the Collector Node.
    ///  For temperature, the Sensor Key is `t` for raw temperature (integer, from 0 to 4095).
    ///  The message will be enqueued for transmission by the CoAP / OIC 
    ///  Background Task so this function will return without waiting for the message 
    ///  to be transmitted.  Return 0 if successful, `SYS_EAGAIN` if network is not ready yet.
    ///  The CoAP payload needs to be very compact (under 32 bytes) so it will be encoded in CBOR like this:
    ///  `{ t: 2870 }`
    fn send_sensor_data_to_collector(sensor_val: &SensorValue,
                                     _node_id: &CStr) -> MynewtResult<()> {
        if unsafe { !NETWORK_IS_READY } {
            return Err(MynewtError::SYS_EAGAIN);
        }
        let rc = unsafe { sensor_network::init_collector_post() };
        if !rc {
            {
                ::core::panicking::panic(&("assertion failed: rc",
                                           "src/send_coap.rs", 348u32, 65u32))
            }
        };
        let _payload =
            {
                "begin cbor root";
                {
                    "begin cbor coap_root";
                    {
                        "begin oc_rep_start_root_object";
                        unsafe {
                            let encoder =
                                JSON_CONTEXT.encoder("root", "_map");
                            tinycbor::cbor_encoder_create_map(JSON_CONTEXT.global_encoder(),
                                                              encoder,
                                                              tinycbor::CborIndefiniteLength)
                        };
                        "end oc_rep_start_root_object";
                    };
                    {
                        " >>  >> sensor_val >> ,";
                        "--------------------";
                        {
                            "begin cbor coap_set_int_val , c : JSON_CONTEXT , val : sensor_val";
                            if let SensorValueType::Uint(val) = sensor_val.val
                                   {
                                "-- cinte c: JSON_CONTEXT, k: sensor_val.key, v: val";
                                let key_with_opt_null: &[u8] =
                                    sensor_val.key.to_bytes_optional_nul();
                                let value = val as i64;
                                "-------------------------------------------------------------";
                                unsafe {
                                    let encoder =
                                        JSON_CONTEXT.encoder("JSON_CONTEXT",
                                                             "_map");
                                    let res =
                                        tinycbor::cbor_encode_text_string(encoder,
                                                                          JSON_CONTEXT.key_to_cstr(key_with_opt_null),
                                                                          JSON_CONTEXT.cstr_len(key_with_opt_null));
                                    JSON_CONTEXT.check_result(res);
                                    let res =
                                        tinycbor::cbor_encode_int(encoder,
                                                                  value);
                                    JSON_CONTEXT.check_result(res);
                                };
                                "-------------------------------------------------------------";
                            } else {
                                unsafe {
                                    JSON_CONTEXT.fail(json_context::JsonError::VALUE_NOT_UINT)
                                };
                            }
                            "end cbor coap_set_int_val";
                        };
                        "--------------------";
                    };
                    {
                        "begin oc_rep_end_root_object";
                        unsafe {
                            let encoder =
                                JSON_CONTEXT.encoder("root", "_map");
                            tinycbor::cbor_encoder_close_container(JSON_CONTEXT.global_encoder(),
                                                                   encoder)
                        };
                        "end oc_rep_end_root_object";
                    };
                    "end cbor coap_root";
                };
                "end cbor root";
                ()
            };
        let rc = unsafe { sensor_network::do_collector_post() };
        if !rc {
            {
                ::core::panicking::panic(&("assertion failed: rc",
                                           "src/send_coap.rs", 359u32, 63u32))
            }
        };
        console_print(b"NRF send to collector: rawtmp %d\n");
        Ok(())
    }
}
use core::panic::PanicInfo;
use cortex_m::asm::bkpt;
use mynewt::kernel::os;
use crate::base::*;
///  main() will be called at Mynewt startup. It replaces the C version of the main() function.
#[no_mangle]
extern "C" fn main() -> ! {
    unsafe { base::rust_sysinit() };
    unsafe { console_flush() };
    send_coap::start_network_task().expect("NET fail");
    listen_sensor::start_sensor_listener().expect("TMP fail");
    loop  { unsafe { os::os_eventq_run(os::os_eventq_dflt_get()) } }
}
///  This function is called on panic, like an assertion failure. We display the filename and line number and pause in the debugger. From https://os.phil-opp.com/freestanding-rust-binary/
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        let file = location.file();
        let line = location.line();
        console_print(b"panic at ");
        unsafe { console_buffer(file.as_ptr(), file.len() as u32) }
        console_print(b" line 0x");
        unsafe { console_printhex(line as u8) }
        console_print(b"\n");
        unsafe { console_flush() }
    } else {
        console_print(b"panic unknown loc\n");
        unsafe { console_flush() }
    }
    bkpt();
    loop  { }
}
