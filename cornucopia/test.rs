#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
mod cli {
    use clap::{Parser, Subcommand};
    use crate::{
        conn, container, error::Error, generate_live, generate_managed, CodegenSettings,
    };
    /// Command line interface to interact with Cornucopia SQL.
    #[clap(version)]
    struct Args {
        /// Use `podman` instead of `docker`
        #[clap(short, long)]
        podman: bool,
        /// Folder containing the queries
        #[clap(short, long, default_value = "queries/")]
        queries_path: String,
        /// Destination folder for generated modules
        #[clap(short, long, default_value = "src/cornucopia.rs")]
        destination: String,
        #[clap(subcommand)]
        action: Action,
        /// Generate synchronous rust code. Async otherwise.
        #[clap(long)]
        sync: bool,
        /// Derive serde's `Serialize` trait for generated types.
        #[clap(long)]
        serialize: bool,
    }
    impl clap::Parser for Args {}
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
    )]
    #[deny(clippy::correctness)]
    #[allow(deprecated)]
    impl clap::CommandFactory for Args {
        fn into_app<'b>() -> clap::Command<'b> {
            let __clap_app = clap::Command::new("cornucopia");
            <Self as clap::Args>::augment_args(__clap_app)
        }
        fn into_app_for_update<'b>() -> clap::Command<'b> {
            let __clap_app = clap::Command::new("cornucopia");
            <Self as clap::Args>::augment_args_for_update(__clap_app)
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for Args {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            let v = Args {
                podman: ::std::convert::From::from(
                    __clap_arg_matches.is_present("podman"),
                ),
                queries_path: __clap_arg_matches
                    .get_one::<String>("queries-path")
                    .map(|s| ::std::ops::Deref::deref(s))
                    .ok_or_else(|| clap::Error::raw(
                        clap::ErrorKind::MissingRequiredArgument,
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["The following required argument was not provided: "],
                                    &[::core::fmt::ArgumentV1::new_display(&"queries-path")],
                                ),
                            );
                            res
                        },
                    ))
                    .and_then(|s| {
                        ::std::str::FromStr::from_str(s)
                            .map_err(|err| clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Invalid value for ", ": "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"queries-path"),
                                                ::core::fmt::ArgumentV1::new_display(&err),
                                            ],
                                        ),
                                    );
                                    res
                                },
                            ))
                    })?,
                destination: __clap_arg_matches
                    .get_one::<String>("destination")
                    .map(|s| ::std::ops::Deref::deref(s))
                    .ok_or_else(|| clap::Error::raw(
                        clap::ErrorKind::MissingRequiredArgument,
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["The following required argument was not provided: "],
                                    &[::core::fmt::ArgumentV1::new_display(&"destination")],
                                ),
                            );
                            res
                        },
                    ))
                    .and_then(|s| {
                        ::std::str::FromStr::from_str(s)
                            .map_err(|err| clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Invalid value for ", ": "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"destination"),
                                                ::core::fmt::ArgumentV1::new_display(&err),
                                            ],
                                        ),
                                    );
                                    res
                                },
                            ))
                    })?,
                action: {
                    <Action as clap::FromArgMatches>::from_arg_matches_mut(
                        __clap_arg_matches,
                    )?
                },
                sync: ::std::convert::From::from(__clap_arg_matches.is_present("sync")),
                serialize: ::std::convert::From::from(
                    __clap_arg_matches.is_present("serialize"),
                ),
            };
            ::std::result::Result::Ok(v)
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if __clap_arg_matches.contains_id("podman") {
                #[allow(non_snake_case)]
                let podman = &mut self.podman;
                *podman = *podman || __clap_arg_matches.is_present("podman");
            }
            if __clap_arg_matches.contains_id("queries-path") {
                #[allow(non_snake_case)]
                let queries_path = &mut self.queries_path;
                *queries_path = __clap_arg_matches
                    .get_one::<String>("queries-path")
                    .map(|s| ::std::ops::Deref::deref(s))
                    .ok_or_else(|| clap::Error::raw(
                        clap::ErrorKind::MissingRequiredArgument,
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["The following required argument was not provided: "],
                                    &[::core::fmt::ArgumentV1::new_display(&"queries-path")],
                                ),
                            );
                            res
                        },
                    ))
                    .and_then(|s| {
                        ::std::str::FromStr::from_str(s)
                            .map_err(|err| clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Invalid value for ", ": "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"queries-path"),
                                                ::core::fmt::ArgumentV1::new_display(&err),
                                            ],
                                        ),
                                    );
                                    res
                                },
                            ))
                    })?;
            }
            if __clap_arg_matches.contains_id("destination") {
                #[allow(non_snake_case)]
                let destination = &mut self.destination;
                *destination = __clap_arg_matches
                    .get_one::<String>("destination")
                    .map(|s| ::std::ops::Deref::deref(s))
                    .ok_or_else(|| clap::Error::raw(
                        clap::ErrorKind::MissingRequiredArgument,
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["The following required argument was not provided: "],
                                    &[::core::fmt::ArgumentV1::new_display(&"destination")],
                                ),
                            );
                            res
                        },
                    ))
                    .and_then(|s| {
                        ::std::str::FromStr::from_str(s)
                            .map_err(|err| clap::Error::raw(
                                clap::ErrorKind::ValueValidation,
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["Invalid value for ", ": "],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&"destination"),
                                                ::core::fmt::ArgumentV1::new_display(&err),
                                            ],
                                        ),
                                    );
                                    res
                                },
                            ))
                    })?;
            }
            {
                #[allow(non_snake_case)]
                let action = &mut self.action;
                <Action as clap::FromArgMatches>::update_from_arg_matches_mut(
                    action,
                    __clap_arg_matches,
                )?;
            }
            if __clap_arg_matches.contains_id("sync") {
                #[allow(non_snake_case)]
                let sync = &mut self.sync;
                *sync = *sync || __clap_arg_matches.is_present("sync");
            }
            if __clap_arg_matches.contains_id("serialize") {
                #[allow(non_snake_case)]
                let serialize = &mut self.serialize;
                *serialize = *serialize || __clap_arg_matches.is_present("serialize");
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
    )]
    #[deny(clippy::correctness)]
    impl clap::Args for Args {
        fn augment_args<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
            {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("podman").takes_value(false);
                        let arg = arg
                            .help("Use `podman` instead of `docker`")
                            .long_help(None)
                            .short('p')
                            .long("podman");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("queries-path")
                            .takes_value(true)
                            .value_name("QUERIES_PATH")
                            .required(
                                false && clap::ArgAction::StoreValue.takes_values(),
                            )
                            .validator(|s| {
                                ::std::str::FromStr::from_str(s).map(|_: String| ())
                            })
                            .value_parser(clap::builder::ValueParser::string())
                            .action(clap::ArgAction::StoreValue);
                        let arg = arg
                            .help("Folder containing the queries")
                            .long_help(None)
                            .short('q')
                            .long("queries-path")
                            .default_value("queries/");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("destination")
                            .takes_value(true)
                            .value_name("DESTINATION")
                            .required(
                                false && clap::ArgAction::StoreValue.takes_values(),
                            )
                            .validator(|s| {
                                ::std::str::FromStr::from_str(s).map(|_: String| ())
                            })
                            .value_parser(clap::builder::ValueParser::string())
                            .action(clap::ArgAction::StoreValue);
                        let arg = arg
                            .help("Destination folder for generated modules")
                            .long_help(None)
                            .short('d')
                            .long("destination")
                            .default_value("src/cornucopia.rs");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("sync").takes_value(false);
                        let arg = arg
                            .help("Generate synchronous rust code. Async otherwise")
                            .long_help(None)
                            .long("sync");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("serialize").takes_value(false);
                        let arg = arg
                            .help("Derive serde's `Serialize` trait for generated types")
                            .long_help(None)
                            .long("serialize");
                        arg
                    });
                let __clap_app = <Action as clap::Subcommand>::augment_subcommands(
                    __clap_app,
                );
                #[allow(deprecated)]
                let __clap_app = __clap_app
                    .setting(clap::AppSettings::SubcommandRequiredElseHelp);
                __clap_app
                    .about("Command line interface to interact with Cornucopia SQL")
                    .long_about(None)
                    .version("0.8.2")
            }
        }
        fn augment_args_for_update<'b>(
            __clap_app: clap::Command<'b>,
        ) -> clap::Command<'b> {
            {
                let __clap_app = __clap_app;
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("podman").takes_value(false);
                        let arg = arg
                            .help("Use `podman` instead of `docker`")
                            .long_help(None)
                            .short('p')
                            .long("podman");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("queries-path")
                            .takes_value(true)
                            .value_name("QUERIES_PATH")
                            .required(
                                false && clap::ArgAction::StoreValue.takes_values(),
                            )
                            .validator(|s| {
                                ::std::str::FromStr::from_str(s).map(|_: String| ())
                            })
                            .value_parser(clap::builder::ValueParser::string())
                            .action(clap::ArgAction::StoreValue);
                        let arg = arg
                            .help("Folder containing the queries")
                            .long_help(None)
                            .short('q')
                            .long("queries-path")
                            .default_value("queries/");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("destination")
                            .takes_value(true)
                            .value_name("DESTINATION")
                            .required(
                                false && clap::ArgAction::StoreValue.takes_values(),
                            )
                            .validator(|s| {
                                ::std::str::FromStr::from_str(s).map(|_: String| ())
                            })
                            .value_parser(clap::builder::ValueParser::string())
                            .action(clap::ArgAction::StoreValue);
                        let arg = arg
                            .help("Destination folder for generated modules")
                            .long_help(None)
                            .short('d')
                            .long("destination")
                            .default_value("src/cornucopia.rs");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("sync").takes_value(false);
                        let arg = arg
                            .help("Generate synchronous rust code. Async otherwise")
                            .long_help(None)
                            .long("sync");
                        arg
                    });
                let __clap_app = __clap_app
                    .arg({
                        #[allow(deprecated)]
                        let arg = clap::Arg::new("serialize").takes_value(false);
                        let arg = arg
                            .help("Derive serde's `Serialize` trait for generated types")
                            .long_help(None)
                            .long("serialize");
                        arg
                    });
                let __clap_app = <Action as clap::Subcommand>::augment_subcommands_for_update(
                    __clap_app,
                );
                __clap_app
                    .about("Command line interface to interact with Cornucopia SQL")
                    .long_about(None)
                    .version("0.8.2")
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Args {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "podman",
                "queries_path",
                "destination",
                "action",
                "sync",
                "serialize",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.podman,
                &&self.queries_path,
                &&self.destination,
                &&self.action,
                &&self.sync,
                &&self.serialize,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Args", names, values)
        }
    }
    enum Action {
        /// Generate your modules against your own db
        Live {
            /// Postgres url to the database
            url: String,
        },
        /// Generate your modules against schema files
        Schema {
            /// SQL files containing the database schema
            schema_files: Vec<String>,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Action {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Action::Live { url: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Live",
                        "url",
                        &__self_0,
                    )
                }
                Action::Schema { schema_files: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Schema",
                        "schema_files",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
    )]
    #[deny(clippy::correctness)]
    impl clap::FromArgMatches for Action {
        fn from_arg_matches(
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            Self::from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn from_arg_matches_mut(
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<Self, clap::Error> {
            #![allow(deprecated)]
            if let Some((__clap_name, mut __clap_arg_sub_matches))
                = __clap_arg_matches.remove_subcommand()
            {
                let __clap_arg_matches = &mut __clap_arg_sub_matches;
                if "live" == __clap_name {
                    return ::std::result::Result::Ok(Action::Live {
                        url: __clap_arg_matches
                            .get_one::<String>("url")
                            .map(|s| ::std::ops::Deref::deref(s))
                            .ok_or_else(|| clap::Error::raw(
                                clap::ErrorKind::MissingRequiredArgument,
                                {
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["The following required argument was not provided: "],
                                            &[::core::fmt::ArgumentV1::new_display(&"url")],
                                        ),
                                    );
                                    res
                                },
                            ))
                            .and_then(|s| {
                                ::std::str::FromStr::from_str(s)
                                    .map_err(|err| clap::Error::raw(
                                        clap::ErrorKind::ValueValidation,
                                        {
                                            let res = ::alloc::fmt::format(
                                                ::core::fmt::Arguments::new_v1(
                                                    &["Invalid value for ", ": "],
                                                    &[
                                                        ::core::fmt::ArgumentV1::new_display(&"url"),
                                                        ::core::fmt::ArgumentV1::new_display(&err),
                                                    ],
                                                ),
                                            );
                                            res
                                        },
                                    ))
                            })?,
                    });
                }
                if "schema" == __clap_name {
                    return ::std::result::Result::Ok(Action::Schema {
                        schema_files: __clap_arg_matches
                            .get_many::<String>("schema-files")
                            .map(|v| {
                                v
                                    .map(|s| ::std::ops::Deref::deref(s))
                                    .map::<
                                        ::std::result::Result<String, clap::Error>,
                                        _,
                                    >(|s| {
                                        ::std::str::FromStr::from_str(s)
                                            .map_err(|err| clap::Error::raw(
                                                clap::ErrorKind::ValueValidation,
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Invalid value for ", ": "],
                                                            &[
                                                                ::core::fmt::ArgumentV1::new_display(&"schema-files"),
                                                                ::core::fmt::ArgumentV1::new_display(&err),
                                                            ],
                                                        ),
                                                    );
                                                    res
                                                },
                                            ))
                                    })
                                    .collect::<::std::result::Result<Vec<_>, clap::Error>>()
                            })
                            .transpose()?
                            .unwrap_or_else(Vec::new),
                    });
                }
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::ErrorKind::UnrecognizedSubcommand,
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["The subcommand \'", "\' wasn\'t recognized"],
                                    &[::core::fmt::ArgumentV1::new_display(&__clap_name)],
                                ),
                            );
                            res
                        },
                    ),
                )
            } else {
                ::std::result::Result::Err(
                    clap::Error::raw(
                        clap::ErrorKind::MissingSubcommand,
                        "A subcommand is required but one was not provided.",
                    ),
                )
            }
        }
        fn update_from_arg_matches(
            &mut self,
            __clap_arg_matches: &clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            self.update_from_arg_matches_mut(&mut __clap_arg_matches.clone())
        }
        fn update_from_arg_matches_mut<'b>(
            &mut self,
            __clap_arg_matches: &mut clap::ArgMatches,
        ) -> ::std::result::Result<(), clap::Error> {
            #![allow(deprecated)]
            if let Some(__clap_name) = __clap_arg_matches.subcommand_name() {
                match self {
                    Action::Live { ref mut url } if "live" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("url") {
                                *url = __clap_arg_matches
                                    .get_one::<String>("url")
                                    .map(|s| ::std::ops::Deref::deref(s))
                                    .ok_or_else(|| clap::Error::raw(
                                        clap::ErrorKind::MissingRequiredArgument,
                                        {
                                            let res = ::alloc::fmt::format(
                                                ::core::fmt::Arguments::new_v1(
                                                    &["The following required argument was not provided: "],
                                                    &[::core::fmt::ArgumentV1::new_display(&"url")],
                                                ),
                                            );
                                            res
                                        },
                                    ))
                                    .and_then(|s| {
                                        ::std::str::FromStr::from_str(s)
                                            .map_err(|err| clap::Error::raw(
                                                clap::ErrorKind::ValueValidation,
                                                {
                                                    let res = ::alloc::fmt::format(
                                                        ::core::fmt::Arguments::new_v1(
                                                            &["Invalid value for ", ": "],
                                                            &[
                                                                ::core::fmt::ArgumentV1::new_display(&"url"),
                                                                ::core::fmt::ArgumentV1::new_display(&err),
                                                            ],
                                                        ),
                                                    );
                                                    res
                                                },
                                            ))
                                    })?;
                            }
                        }
                    }
                    Action::Schema {
                        ref mut schema_files,
                    } if "schema" == __clap_name => {
                        let (_, mut __clap_arg_sub_matches) = __clap_arg_matches
                            .remove_subcommand()
                            .unwrap();
                        let __clap_arg_matches = &mut __clap_arg_sub_matches;
                        {
                            if __clap_arg_matches.contains_id("schema-files") {
                                *schema_files = __clap_arg_matches
                                    .get_many::<String>("schema-files")
                                    .map(|v| {
                                        v
                                            .map(|s| ::std::ops::Deref::deref(s))
                                            .map::<
                                                ::std::result::Result<String, clap::Error>,
                                                _,
                                            >(|s| {
                                                ::std::str::FromStr::from_str(s)
                                                    .map_err(|err| clap::Error::raw(
                                                        clap::ErrorKind::ValueValidation,
                                                        {
                                                            let res = ::alloc::fmt::format(
                                                                ::core::fmt::Arguments::new_v1(
                                                                    &["Invalid value for ", ": "],
                                                                    &[
                                                                        ::core::fmt::ArgumentV1::new_display(&"schema-files"),
                                                                        ::core::fmt::ArgumentV1::new_display(&err),
                                                                    ],
                                                                ),
                                                            );
                                                            res
                                                        },
                                                    ))
                                            })
                                            .collect::<::std::result::Result<Vec<_>, clap::Error>>()
                                    })
                                    .transpose()?
                                    .unwrap_or_else(Vec::new);
                            }
                        }
                    }
                    s => {
                        *s = <Self as clap::FromArgMatches>::from_arg_matches_mut(
                            __clap_arg_matches,
                        )?;
                    }
                }
            }
            ::std::result::Result::Ok(())
        }
    }
    #[allow(dead_code, unreachable_code, unused_variables, unused_braces)]
    #[allow(
        clippy::style,
        clippy::complexity,
        clippy::pedantic,
        clippy::restriction,
        clippy::perf,
        clippy::deprecated,
        clippy::nursery,
        clippy::cargo,
        clippy::suspicious_else_formatting,
    )]
    #[deny(clippy::correctness)]
    impl clap::Subcommand for Action {
        fn augment_subcommands<'b>(__clap_app: clap::Command<'b>) -> clap::Command<'b> {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("live");
                    {
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("url")
                                    .takes_value(true)
                                    .value_name("URL")
                                    .required(
                                        true && clap::ArgAction::StoreValue.takes_values(),
                                    )
                                    .validator(|s| {
                                        ::std::str::FromStr::from_str(s).map(|_: String| ())
                                    })
                                    .value_parser(clap::builder::ValueParser::string())
                                    .action(clap::ArgAction::StoreValue);
                                let arg = arg
                                    .help("Postgres url to the database")
                                    .long_help(None);
                                arg
                            });
                        __clap_subcommand
                            .about("Generate your modules against your own db")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("schema");
                    {
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("schema-files")
                                    .takes_value(true)
                                    .value_name("SCHEMA_FILES")
                                    .multiple_occurrences(true)
                                    .validator(|s| {
                                        ::std::str::FromStr::from_str(s).map(|_: String| ())
                                    })
                                    .value_parser(clap::builder::ValueParser::string())
                                    .action(clap::ArgAction::StoreValue);
                                let arg = arg
                                    .help("SQL files containing the database schema")
                                    .long_help(None);
                                arg
                            });
                        __clap_subcommand
                            .about("Generate your modules against schema files")
                            .long_about(None)
                    }
                });
            __clap_app
        }
        fn augment_subcommands_for_update<'b>(
            __clap_app: clap::Command<'b>,
        ) -> clap::Command<'b> {
            let __clap_app = __clap_app;
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("live");
                    {
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("url")
                                    .takes_value(true)
                                    .value_name("URL")
                                    .required(
                                        false && clap::ArgAction::StoreValue.takes_values(),
                                    )
                                    .validator(|s| {
                                        ::std::str::FromStr::from_str(s).map(|_: String| ())
                                    })
                                    .value_parser(clap::builder::ValueParser::string())
                                    .action(clap::ArgAction::StoreValue);
                                let arg = arg
                                    .help("Postgres url to the database")
                                    .long_help(None);
                                arg
                            });
                        __clap_subcommand
                            .about("Generate your modules against your own db")
                            .long_about(None)
                    }
                });
            let __clap_app = __clap_app
                .subcommand({
                    let __clap_subcommand = clap::Command::new("schema");
                    {
                        let __clap_subcommand = __clap_subcommand;
                        let __clap_subcommand = __clap_subcommand
                            .arg({
                                #[allow(deprecated)]
                                let arg = clap::Arg::new("schema-files")
                                    .takes_value(true)
                                    .value_name("SCHEMA_FILES")
                                    .multiple_occurrences(true)
                                    .validator(|s| {
                                        ::std::str::FromStr::from_str(s).map(|_: String| ())
                                    })
                                    .value_parser(clap::builder::ValueParser::string())
                                    .action(clap::ArgAction::StoreValue);
                                let arg = arg
                                    .help("SQL files containing the database schema")
                                    .long_help(None);
                                arg
                            });
                        __clap_subcommand
                            .about("Generate your modules against schema files")
                            .long_about(None)
                    }
                });
            __clap_app
        }
        fn has_subcommand(__clap_name: &str) -> bool {
            if "live" == __clap_name {
                return true;
            }
            if "schema" == __clap_name {
                return true;
            }
            false
        }
    }
    pub fn run() -> Result<(), Error> {
        let Args { podman, queries_path, destination, action, sync, serialize } = Args::parse();
        match action {
            Action::Live { url } => {
                let mut client = conn::from_url(&url)?;
                generate_live(
                    &mut client,
                    &queries_path,
                    Some(&destination),
                    CodegenSettings {
                        is_async: !sync,
                        derive_ser: serialize,
                    },
                )?;
            }
            Action::Schema { schema_files } => {
                if let Err(e)
                    = generate_managed(
                        &queries_path,
                        schema_files,
                        Some(&destination),
                        podman,
                        CodegenSettings {
                            is_async: !sync,
                            derive_ser: serialize,
                        },
                    ) {
                    container::cleanup(podman).ok();
                    return Err(e);
                }
            }
        };
        Ok(())
    }
}
mod codegen {
    use core::str;
    use std::fmt::{Formatter, Write};
    use heck::ToUpperCamelCase;
    use indexmap::IndexMap;
    use quote::quote;
    use crate::{
        prepare_queries::{
            Preparation, PreparedContent, PreparedField, PreparedItem, PreparedModule,
            PreparedQuery, PreparedType,
        },
        utils::{escape_keyword, unescape_keyword, Lazy},
        CodegenSettings,
    };
    impl PreparedField {
        pub fn own_struct(&self) -> String {
            let it = self.ty.own_ty(self.is_inner_nullable);
            if self.is_nullable {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Option<", ">"],
                            &[::core::fmt::ArgumentV1::new_display(&it)],
                        ),
                    );
                    res
                }
            } else {
                it
            }
        }
        pub fn param_ergo_ty(&self, is_async: bool, traits: &mut Vec<String>) -> String {
            let it = self.ty.param_ergo_ty(self.is_inner_nullable, is_async, traits);
            if self.is_nullable {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Option<", ">"],
                            &[::core::fmt::ArgumentV1::new_display(&it)],
                        ),
                    );
                    res
                }
            } else {
                it
            }
        }
        pub fn param_ty(&self, is_async: bool) -> String {
            let it = self.ty.param_ty(self.is_inner_nullable, is_async);
            if self.is_nullable {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Option<", ">"],
                            &[::core::fmt::ArgumentV1::new_display(&it)],
                        ),
                    );
                    res
                }
            } else {
                it
            }
        }
        pub fn brw_ty(&self, has_lifetime: bool, is_async: bool) -> String {
            let it = self.ty.brw_ty(self.is_inner_nullable, has_lifetime, is_async);
            if self.is_nullable {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["Option<", ">"],
                            &[::core::fmt::ArgumentV1::new_display(&it)],
                        ),
                    );
                    res
                }
            } else {
                it
            }
        }
        pub fn owning_call(&self, name: Option<&str>) -> String {
            self.ty
                .owning_call(
                    name.unwrap_or(&self.name),
                    self.is_nullable,
                    self.is_inner_nullable,
                )
        }
        pub fn owning_assign(&self) -> String {
            let call = self.owning_call(None);
            if call == self.name {
                call
            } else {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", ": "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&self.name),
                                ::core::fmt::ArgumentV1::new_display(&call),
                            ],
                        ),
                    );
                    res
                }
            }
        }
    }
    fn enum_sql(w: &mut impl Write, name: &str, enum_name: &str, variants: &[String]) {
        let enum_names = std::iter::repeat(enum_name);
        let enum_names2 = enum_names.clone();
        let unescaped_variants_str: Vec<_> = variants
            .iter()
            .map(|v| {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["\"", "\""],
                        &[::core::fmt::ArgumentV1::new_display(&unescape_keyword(v))],
                    ),
                );
                res
            })
            .collect();
        let name = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["\"", "\""],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                ),
            );
            res
        };
        let nb_variants = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["", "usize"],
                    &[::core::fmt::ArgumentV1::new_display(&variants.len())],
                ),
            );
            res
        };
        {
            w.write_str("impl < 'a > postgres_types :: ToSql for ").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&enum_name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    "
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, buf : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let s = match * self
        { ",
                )
                .unwrap();
            {
                let iter = enum_names
                    .into_iter()
                    .zip(variants.into_iter())
                    .zip(unescaped_variants_str.into_iter());
                let sep = '*';
                let first = true;
                for (enum_names, variants, unescaped_variants_str) in iter {}
            }
            w.write_str(
                    " } ;
        buf.extend_from_slice(s.as_bytes()) ; std :: result :: Result ::
        Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&nb_variants)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; }
                variants.iter().all(| v | match & * * v
                { ",
                )
                .unwrap();
            {
                let iter = unescaped_variants_str.into_iter();
                let sep = '*';
                let first = true;
                for (unescaped_variants_str,) in iter {}
            }
            w.write_str(
                    " _ => false, })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
} impl < 'a > postgres_types :: FromSql < 'a > for ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&enum_name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    "
{
    fn from_sql(ty : & postgres_types :: Type, buf : & 'a [u8],) -> Result < ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&enum_name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    ", Box < dyn std :: error :: Error + Sync + Send >, >
    {
        match std :: str :: from_utf8(buf) ?
        {
            ",
                )
                .unwrap();
            {
                let iter = unescaped_variants_str
                    .into_iter()
                    .zip(enum_names2.into_iter())
                    .zip(variants.into_iter());
                let sep = '*';
                let first = true;
                for (unescaped_variants_str, enum_names2, variants) in iter {}
            }
            w.write_str(
                    "
            s => Result ::
            Err(Into :: into(format! (\"invalid variant `{}`\", s))),
        }
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Enum(ref variants) =>
            {
                if variants.len() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&nb_variants)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; }
                variants.iter().all(| v | match & * * v
                { ",
                )
                .unwrap();
            {
                let iter = unescaped_variants_str.into_iter();
                let sep = '*';
                let first = true;
                for (unescaped_variants_str,) in iter {}
            }
            w.write_str(" _ => false, })
            } _ => false,
        }
    }
}")
                .unwrap();
        };
    }
    fn struct_tosql(
        w: &mut impl Write,
        struct_name: &str,
        fields: &[PreparedField],
        name: &str,
        is_borrow: bool,
        is_params: bool,
        is_async: bool,
    ) {
        let (struct_name, lifetime) = if is_borrow {
            if is_params {
                (
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "Borrowed"],
                                &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                            ),
                        );
                        res
                    },
                    "<'a>",
                )
            } else {
                (
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "Params"],
                                &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                            ),
                        );
                        res
                    },
                    "<'a>",
                )
            }
        } else {
            (struct_name.to_string(), "")
        };
        let field_names = fields.iter().map(|p| &p.name);
        let write_names = fields
            .iter()
            .map(|p| {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["\"", "\""],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &unescape_keyword(&p.name),
                            ),
                        ],
                    ),
                );
                res
            });
        let write_ty = fields.iter().map(|p| p.ty.sql_wrapped(&p.name, is_async));
        let accept_names = write_names.clone();
        let accept_ty = fields.iter().map(|p| p.ty.accept_to_sql(is_async));
        let name = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["\"", "\""],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                ),
            );
            res
        };
        let nb_fields = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["", "usize"],
                    &[::core::fmt::ArgumentV1::new_display(&fields.len())],
                ),
            );
            res
        };
        {
            w.write_str("impl < 'a > postgres_types :: ToSql for ").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                    ),
                )
                .unwrap();
            w.write_str(" ").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                    ),
                )
                .unwrap();
            w.write_str(
                    "
{
    fn
    to_sql(& self, ty : & postgres_types :: Type, out : & mut postgres_types
    :: private :: BytesMut,) -> Result < postgres_types :: IsNull, Box < dyn
    std :: error :: Error + Sync + Send >, >
    {
        let ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                    ),
                )
                .unwrap();
            w.write_str(" { ").unwrap();
            {
                let iter = field_names.into_iter();
                let sep = ',';
                let first = true;
                for (field_names,) in iter {}
            }
            w.write_str(
                    " } = self ; let fields = match
        * ty.kind()
        {
            postgres_types :: Kind :: Composite(ref fields) => fields, _ =>
            unreachable! (),
        } ; out.extend_from_slice(& (fields.len() as i32).to_be_bytes()) ; for
        field in fields
        {
            out.extend_from_slice(& field.type_().oid().to_be_bytes()) ; let
            base = out.len() ; out.extend_from_slice(& [0 ; 4]) ; let r =
            match field.name()
            {
                ",
                )
                .unwrap();
            {
                let iter = write_names.into_iter().zip(write_ty.into_iter());
                let sep = '*';
                let first = true;
                for (write_names, write_ty) in iter {}
            }
            w.write_str(
                    " _ => unreachable!
                ()
            } ; let count = match r ?
            {
                postgres_types :: IsNull :: Yes => - 1, postgres_types ::
                IsNull :: No =>
                {
                    let len = out.len() - base - 4 ; if len > i32 :: max_value()
                    as usize
                    {
                        return Err(Into :: into(\"value too large to transmit\")) ;
                    } len as i32
                }
            } ; out [base .. base + 4].copy_from_slice(& count.to_be_bytes())
            ;
        } Ok(postgres_types :: IsNull :: No)
    } fn accepts(ty : & postgres_types :: Type) -> bool
    {
        if ty.name() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; } match * ty.kind()
        {
            postgres_types :: Kind :: Composite(ref fields) =>
            {
                if fields.len() != ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&nb_fields)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " { return false ; }
                fields.iter().all(| f | match f.name()
                {
                    ",
                )
                .unwrap();
            {
                let iter = accept_names.into_iter().zip(accept_ty.into_iter());
                let sep = '*';
                let first = true;
                for (accept_names, accept_ty) in iter {}
            }
            w.write_str(
                    " _ => false,
                })
            } _ => false,
        }
    } fn
    to_sql_checked(& self, ty : & postgres_types :: Type, out : & mut
    postgres_types :: private :: BytesMut,) -> Result < postgres_types ::
    IsNull, Box < dyn std :: error :: Error + Sync + Send >>
    { postgres_types :: __to_sql_checked(self, ty, out) }
}",
                )
                .unwrap();
        };
    }
    fn composite_fromsql(
        w: &mut impl Write,
        struct_name: &str,
        fields: &[PreparedField],
        name: &str,
        schema: &str,
    ) {
        let field_names = fields.iter().map(|p| &p.name);
        let read_names = field_names.clone();
        let read_idx = 0..fields.len();
        let struct_name = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["", "Borrowed"],
                    &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                ),
            );
            res
        };
        let name = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["\"", "\""],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                ),
            );
            res
        };
        let schema = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["\"", "\""],
                    &[::core::fmt::ArgumentV1::new_display(&schema)],
                ),
            );
            res
        };
        {
            w.write_str("impl < 'a > postgres_types :: FromSql < 'a > for ").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " < 'a >
{
    fn from_sql(ty : & postgres_types :: Type, out : & 'a [u8]) -> Result < ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                    ),
                )
                .unwrap();
            w.write_str(
                    " < 'a >, Box < dyn std :: error :: Error + Sync + Send >>
    {
        let fields = match * ty.kind()
        {
            postgres_types :: Kind :: Composite(ref fields) => fields, _ =>
            unreachable! (),
        } ; let mut out = out ; let num_fields = postgres_types :: private ::
        read_be_i32(& mut out) ? ; if num_fields as usize != fields.len()
        {
            return std :: result :: Result ::
            Err(std :: convert :: Into ::
            into(format!
            (\"invalid field count: {} vs {}\", num_fields, fields.len()))) ;
        } ",
                )
                .unwrap();
            {
                let iter = read_names.into_iter().zip(read_idx.into_iter());
                let sep = '*';
                let first = true;
                for (read_names, read_idx) in iter {}
            }
            w.write_str("
        Ok(").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                    ),
                )
                .unwrap();
            w.write_str(" { ").unwrap();
            {
                let iter = field_names.into_iter();
                let sep = ',';
                let first = true;
                for (field_names,) in iter {}
            }
            w.write_str(
                    " })
    } fn accepts(ty : & postgres_types :: Type) -> bool
    { ty.name() == ",
                )
                .unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&name)],
                    ),
                )
                .unwrap();
            w.write_str(" && ty.schema() == ").unwrap();
            w.write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[""],
                        &[::core::fmt::ArgumentV1::new_display(&schema)],
                    ),
                )
                .unwrap();
            w.write_str(" }
}").unwrap();
        };
    }
    fn gen_params_struct(
        w: &mut impl Write,
        params: &PreparedItem,
        settings: CodegenSettings,
    ) {
        let PreparedItem { name, fields, is_copy, is_named, is_ref } = params;
        let is_async = settings.is_async;
        if *is_named {
            let name = name.to_string();
            let traits = &mut Vec::new();
            let copy = if *is_copy { "Clone,Copy," } else { "" };
            let lifetime = if *is_ref { "'a," } else { "" };
            let fields_ty = fields
                .iter()
                .map(|p| p.param_ergo_ty(is_async, traits))
                .collect::<Vec<_>>();
            let fields_name = fields.iter().map(|p| &p.name);
            let traits_idx = (1..=traits.len()).into_iter().map(idx_char);
            {
                w.write_str("#[derive(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&copy)],
                        ),
                    )
                    .unwrap();
                w.write_str(" Debug)] pub struct ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str(" < ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                        ),
                    )
                    .unwrap();
                w.write_str(" ").unwrap();
                {
                    let iter = traits_idx.into_iter().zip(traits.into_iter());
                    let sep = ',';
                    let first = true;
                    for (traits_idx, traits) in iter {}
                }
                w.write_str(" > { ").unwrap();
                {
                    let iter = fields_name.into_iter().zip(fields_ty.into_iter());
                    let sep = ',';
                    let first = true;
                    for (fields_name, fields_ty) in iter {}
                }
                w.write_str(" }").unwrap();
            };
        }
    }
    fn gen_row_structs(
        w: &mut impl Write,
        row: &PreparedItem,
        CodegenSettings { is_async, derive_ser }: CodegenSettings,
    ) {
        let PreparedItem { name, fields, is_copy, is_named, .. } = row;
        if *is_named {
            let fields_name = fields.iter().map(|p| &p.name);
            let fields_ty = fields.iter().map(|p| p.own_struct());
            let copy = if *is_copy { "Copy" } else { "" };
            let ser_str = if derive_ser { "serde::Serialize," } else { "" };
            {
                w.write_str("#[derive(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&ser_str)],
                        ),
                    )
                    .unwrap();
                w.write_str(" Debug, Clone, PartialEq, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&copy)],
                        ),
                    )
                    .unwrap();
                w.write_str(")] pub struct ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str("
{ ").unwrap();
                {
                    let iter = fields_name.into_iter().zip(fields_ty.into_iter());
                    let sep = ',';
                    let first = true;
                    for (fields_name, fields_ty) in iter {}
                }
                w.write_str(" }").unwrap();
            };
            if !is_copy {
                let fields_name = fields.iter().map(|p| &p.name);
                let fields_ty = fields.iter().map(|p| p.brw_ty(true, is_async));
                let from_name = fields_name.clone();
                let from_own_assign = fields.iter().map(|f| f.owning_assign());
                let brw_name = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", "Borrowed"],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    );
                    res
                };
                {
                    w.write_str("pub struct ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" < 'a > { ").unwrap();
                    {
                        let iter = fields_name.into_iter().zip(fields_ty.into_iter());
                        let sep = ',';
                        let first = true;
                        for (fields_name, fields_ty) in iter {}
                    }
                    w.write_str(" }
impl < 'a > From < ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" < 'a >> for ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&name)],
                            ),
                        )
                        .unwrap();
                    w.write_str("
{
    fn from(").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" { ").unwrap();
                    {
                        let iter = from_name.into_iter();
                        let sep = ',';
                        let first = true;
                        for (from_name,) in iter {}
                    }
                    w.write_str(" } : ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" < 'a >) -> Self
    { Self { ").unwrap();
                    {
                        let iter = from_own_assign.into_iter();
                        let sep = ',';
                        let first = true;
                        for (from_own_assign,) in iter {}
                    }
                    w.write_str(" } }
}").unwrap();
                };
            }
        }
        {
            let borrowed_str = if *is_copy { "" } else { "Borrowed" };
            let (
                client_mut,
                fn_async,
                fn_await,
                backend,
                collect,
                raw_type,
                raw_pre,
                raw_post,
                client,
            ) = if is_async {
                (
                    "",
                    "async",
                    ".await",
                    "tokio_postgres",
                    "try_collect().await",
                    "futures::Stream",
                    "",
                    ".into_stream()",
                    "cornucopia_async",
                )
            } else {
                (
                    "mut",
                    "",
                    "",
                    "postgres",
                    "collect()",
                    "Iterator",
                    ".iterator()",
                    "",
                    "cornucopia_sync",
                )
            };
            let row_struct = if *is_named {
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", ""],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&name),
                                ::core::fmt::ArgumentV1::new_display(&borrowed_str),
                            ],
                        ),
                    );
                    res
                }
            } else {
                fields[0].brw_ty(false, is_async)
            };
            let name = {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["", "Query"],
                        &[::core::fmt::ArgumentV1::new_display(&name)],
                    ),
                );
                res
            };
            {
                w.write_str("pub struct ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " < 'a, C : GenericClient, T, const N : usize >
{
    client : & 'a ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&client_mut)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " C, params :
    [& 'a (dyn postgres_types :: ToSql + Sync) ; N], stmt : & 'a mut ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&client)],
                        ),
                    )
                    .unwrap();
                w.write_str("
    :: private :: Stmt, extractor : fn(& ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: Row) -> ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&row_struct)],
                        ),
                    )
                    .unwrap();
                w.write_str(",
    mapper : fn(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&row_struct)],
                        ),
                    )
                    .unwrap();
                w.write_str(") -> T,
} impl < 'a, C, T : 'a, const N : usize > ")
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " < 'a, C, T, N > where C :
GenericClient
{
    pub fn map < R > (self, mapper : fn(",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&row_struct)],
                        ),
                    )
                    .unwrap();
                w.write_str(") -> R) -> ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str(" < 'a, C,
    R, N >
    {
        ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        "
        {
            client : self.client, params : self.params, stmt : self.stmt,
            extractor : self.extractor, mapper,
        }
    } pub ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_async)],
                        ),
                    )
                    .unwrap();
                w.write_str(" fn one(self) -> Result < T, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " :: Error >
    {
        let stmt = self.stmt.prepare(self.client) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " ? ; let row =
        self.client.query_one(stmt, & self.params) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " ? ;
        Ok((self.mapper) ((self.extractor) (& row)))
    } pub ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_async)],
                        ),
                    )
                    .unwrap();
                w.write_str(" fn all(self) -> Result < Vec < T >, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: Error >
    { self.iter() ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(" ?.").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&collect)],
                        ),
                    )
                    .unwrap();
                w.write_str(" } pub ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_async)],
                        ),
                    )
                    .unwrap();
                w.write_str(" fn opt(self) ->
    Result < Option < T >, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " :: Error >
    {
        let stmt = self.stmt.prepare(self.client) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " ? ;
        Ok(self.client.query_opt(stmt, & self.params) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        "
        ?.map(| row | (self.mapper) ((self.extractor) (& row))))
    } pub ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_async)],
                        ),
                    )
                    .unwrap();
                w.write_str(" fn iter(self,) -> Result < impl ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&raw_type)],
                        ),
                    )
                    .unwrap();
                w.write_str(" < Item =
    Result < T, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: Error >> + 'a, ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&backend)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        " :: Error >
    {
        let stmt = self.stmt.prepare(self.client) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(" ? ; let it =
        self.client.query_raw(stmt, ")
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&client)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: private ::
        slice_iter(& self.params)) ")
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                        ),
                    )
                    .unwrap();
                w.write_str(" ? ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&raw_pre)],
                        ),
                    )
                    .unwrap();
                w.write_str(
                        ".map(move | res |
        res.map(| row | (self.mapper) ((self.extractor) (& row)))) ",
                    )
                    .unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&raw_post)],
                        ),
                    )
                    .unwrap();
                w.write_str("
        ; Ok(it)
    }
}").unwrap();
            };
        }
    }
    pub fn idx_char(idx: usize) -> String {
        {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["T"],
                    &[::core::fmt::ArgumentV1::new_display(&idx)],
                ),
            );
            res
        }
    }
    fn gen_query_fn(
        w: &mut impl Write,
        module: &PreparedModule,
        query: &PreparedQuery,
        CodegenSettings { is_async, .. }: CodegenSettings,
    ) {
        let PreparedQuery { name, row, sql, param } = query;
        let (client_mut, fn_async, fn_await, backend, client) = if is_async {
            ("", "async", ".await", "tokio_postgres", "cornucopia_async")
        } else {
            ("mut", "", "", "postgres", "cornucopia_sync")
        };
        let struct_name = name.to_upper_camel_case();
        let stmt_name = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["", "Stmt"],
                    &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                ),
            );
            res
        };
        let (param, param_field, order) = match param {
            Some((idx, order)) => {
                let it = module.params.get_index(*idx).unwrap().1;
                (Some(it), it.fields.as_slice(), order.as_slice())
            }
            None => (None, [].as_slice(), [].as_slice()),
        };
        let traits = &mut Vec::new();
        let params_ty: Vec<_> = order
            .iter()
            .map(|idx| param_field[*idx].param_ergo_ty(is_async, traits))
            .collect();
        let params_name = order.iter().map(|idx| &param_field[*idx].name);
        let traits_idx = (1..=traits.len()).into_iter().map(idx_char);
        let lazy_impl = Lazy::new(|w| {
            if let Some((idx, index)) = row {
                let PreparedItem { name: row_name, fields, is_copy, is_named, .. } = &module
                    .rows
                    .get_index(*idx)
                    .unwrap()
                    .1;
                let params_name = params_name.clone();
                let params_name2 = params_name.clone();
                let nb_params = param_field.len();
                let traits_idx = traits_idx.clone();
                let (
                    row_struct_name,
                    extractor,
                    mapper,
                ): (String, Lazy<Box<dyn Fn(&mut Formatter)>>, String) = if *is_named {
                    (
                        row_name.value.clone(),
                        Lazy::new(
                            Box::new(|w: &mut Formatter| {
                                let name = if *is_copy {
                                    row_name.to_string()
                                } else {
                                    {
                                        let res = ::alloc::fmt::format(
                                            ::core::fmt::Arguments::new_v1(
                                                &["", "Borrowed"],
                                                &[::core::fmt::ArgumentV1::new_display(&row_name)],
                                            ),
                                        );
                                        res
                                    }
                                };
                                let fields_name = fields.iter().map(|p| &p.name);
                                let fields_idx = (0..fields.len()).map(|i| index[i]);
                                {
                                    w.write_fmt(
                                            ::core::fmt::Arguments::new_v1(
                                                &[""],
                                                &[::core::fmt::ArgumentV1::new_display(&name)],
                                            ),
                                        )
                                        .unwrap();
                                    w.write_str(" { ").unwrap();
                                    {
                                        let iter = fields_name
                                            .into_iter()
                                            .zip(fields_idx.into_iter());
                                        let sep = ',';
                                        let first = true;
                                        for (fields_name, fields_idx) in iter {}
                                    }
                                    w.write_str(" }").unwrap();
                                }
                            }),
                        ),
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["<", ">::from(it)"],
                                    &[::core::fmt::ArgumentV1::new_display(&row_name)],
                                ),
                            );
                            res
                        },
                    )
                } else {
                    let field = &fields[0];
                    (
                        field.own_struct(),
                        Lazy::new(
                            Box::new(|w: &mut Formatter| {
                                w.write_str("row.get(0)").unwrap();
                            }),
                        ),
                        field.owning_call(Some("it")),
                    )
                };
                let query_name = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", "Query"],
                            &[::core::fmt::ArgumentV1::new_display(&row_name)],
                        ),
                    );
                    res
                };
                {
                    w.write_str("pub fn bind < 'a, C : GenericClient, ").unwrap();
                    {
                        let iter = traits_idx.into_iter().zip(traits.into_iter());
                        let sep = ',';
                        let first = true;
                        for (traits_idx, traits) in iter {}
                    }
                    w.write_str(" >
(& 'a mut self, client : & 'a ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&client_mut)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" C, ").unwrap();
                    {
                        let iter = params_name.into_iter().zip(params_ty.into_iter());
                        let sep = ',';
                        let first = true;
                        for (params_name, params_ty) in iter {}
                    }
                    w.write_str(") -> ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&query_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" < 'a, C, ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&row_struct_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(", ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&nb_params)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" >
{
    ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&query_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str("
    {
        client, params : [").unwrap();
                    {
                        let iter = params_name2.into_iter();
                        let sep = ',';
                        let first = true;
                        for (params_name2,) in iter {}
                    }
                    w.write_str("], stmt : & mut self.0,
        extractor : | row | { ")
                        .unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&extractor)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" }, mapper : | it | { ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&mapper)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" },
    }
}").unwrap();
                };
            } else {
                let params_wrap = order
                    .iter()
                    .map(|idx| {
                        let p = &param_field[*idx];
                        p.ty.sql_wrapped(&p.name, is_async)
                    });
                let traits_idx = traits_idx.clone();
                let params_name = params_name.clone();
                {
                    w.write_str("pub ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&fn_async)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" fn bind < 'a, C : GenericClient, ").unwrap();
                    {
                        let iter = traits_idx.into_iter().zip(traits.into_iter());
                        let sep = ',';
                        let first = true;
                        for (traits_idx, traits) in iter {}
                    }
                    w.write_str(" >
(& 'a mut self, client : & 'a ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&client_mut)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" C, ").unwrap();
                    {
                        let iter = params_name.into_iter().zip(params_ty.into_iter());
                        let sep = ',';
                        let first = true;
                        for (params_name, params_ty) in iter {}
                    }
                    w.write_str(") -> Result < u64, ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&backend)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" :: Error >
{
    let stmt = self.0.prepare(client) ")
                        .unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" ? ;
    client.execute(stmt, & [").unwrap();
                    {
                        let iter = params_wrap.into_iter();
                        let sep = ',';
                        let first = true;
                        for (params_wrap,) in iter {}
                    }
                    w.write_str("]) ").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&fn_await)],
                            ),
                        )
                        .unwrap();
                    w.write_str("
}").unwrap();
                };
            }
        });
        {
            let sql = sql.replace('"', "\\\"");
            let sql = {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["\"", "\""],
                        &[::core::fmt::ArgumentV1::new_display(&sql)],
                    ),
                );
                res
            };
            let name = escape_keyword(name.clone());
            {
                w.write_str("pub fn ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    )
                    .unwrap();
                w.write_str("() -> ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                        ),
                    )
                    .unwrap();
                w.write_str("
{ ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                        ),
                    )
                    .unwrap();
                w.write_str("(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&client)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: private :: Stmt :: new(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&sql)],
                        ),
                    )
                    .unwrap();
                w.write_str(")) } pub struct ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                        ),
                    )
                    .unwrap();
                w.write_str("(").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&client)],
                        ),
                    )
                    .unwrap();
                w.write_str(" :: private :: Stmt) ; impl ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                        ),
                    )
                    .unwrap();
                w.write_str(" { ").unwrap();
                w.write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&lazy_impl)],
                        ),
                    )
                    .unwrap();
                w.write_str(" }").unwrap();
            };
        }
        if let Some(param) = param {
            let traits_idx2 = traits_idx.clone();
            let traits_idx3 = traits_idx.clone();
            if param.is_named {
                let param_name = &param.name;
                let lifetime = if param.is_copy || !param.is_ref { "" } else { "'a," };
                if let Some((idx, _)) = row {
                    let prepared_row = &module.rows.get_index(*idx).unwrap().1;
                    let name = prepared_row.name.value.clone();
                    let query_row_struct = if prepared_row.is_named {
                        name
                    } else {
                        prepared_row.fields[0].own_struct()
                    };
                    let name = &module.rows.get_index(*idx).unwrap().1.name;
                    let nb_params = param_field.len();
                    let query_name = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "Query"],
                                &[::core::fmt::ArgumentV1::new_display(&name)],
                            ),
                        );
                        res
                    };
                    {
                        w.write_str("impl < 'a, C : GenericClient, ").unwrap();
                        {
                            let iter = traits_idx.into_iter().zip(traits.into_iter());
                            let sep = ',';
                            let first = true;
                            for (traits_idx, traits) in iter {}
                        }
                        w.write_str(" > ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&client)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
:: Params < 'a, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&param_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" ").unwrap();
                        {
                            let iter = traits_idx2.into_iter();
                            let sep = ',';
                            let first = true;
                            for (traits_idx2,) in iter {}
                        }
                        w.write_str(" >, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&query_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
< 'a, C, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&query_row_struct)],
                                ),
                            )
                            .unwrap();
                        w.write_str(", ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&nb_params)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" >, C > for ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
{
    fn
    params(& 'a mut self, client : & 'a ")
                            .unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&client_mut)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" C, params : & 'a ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&param_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" ").unwrap();
                        {
                            let iter = traits_idx3.into_iter();
                            let sep = ',';
                            let first = true;
                            for (traits_idx3,) in iter {}
                        }
                        w.write_str(" >) -> ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&query_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < 'a, C, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&query_row_struct)],
                                ),
                            )
                            .unwrap();
                        w.write_str(", ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&nb_params)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" >
    { self.bind(client, ").unwrap();
                        {
                            let iter = params_name.into_iter();
                            let sep = ',';
                            let first = true;
                            for (params_name,) in iter {}
                        }
                        w.write_str(") }
}").unwrap();
                    };
                } else {
                    let (send_sync, pre_ty, post_ty_lf, pre, post) = if is_async {
                        (
                            "+ Send + Sync",
                            "std::pin::Pin<Box<dyn futures::Future<Output = Result",
                            "> + Send + 'a>>",
                            "Box::pin(self",
                            ")",
                        )
                    } else {
                        ("", "Result", "", "self", "")
                    };
                    {
                        w.write_str("impl < 'a, C : GenericClient ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&send_sync)],
                                ),
                            )
                            .unwrap();
                        w.write_str(", ").unwrap();
                        {
                            let iter = traits_idx.into_iter().zip(traits.into_iter());
                            let sep = ',';
                            let first = true;
                            for (traits_idx, traits) in iter {}
                        }
                        w.write_str("
> ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&client)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" :: Params < 'a, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&param_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" ").unwrap();
                        {
                            let iter = traits_idx2.into_iter();
                            let sep = ',';
                            let first = true;
                            for (traits_idx2,) in iter {}
                        }
                        w.write_str(" >, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&pre_ty)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < u64, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&backend)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" :: Error > ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&post_ty_lf)],
                                ),
                            )
                            .unwrap();
                        w.write_str(", C > for ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&stmt_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
{
    fn
    params(& 'a mut self, client : & 'a ")
                            .unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&client_mut)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" C, params : & 'a ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&param_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" ").unwrap();
                        {
                            let iter = traits_idx3.into_iter();
                            let sep = ',';
                            let first = true;
                            for (traits_idx3,) in iter {}
                        }
                        w.write_str(" >) -> ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&pre_ty)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < u64, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&backend)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" :: Error > ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&post_ty_lf)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
    { ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&pre)],
                                ),
                            )
                            .unwrap();
                        w.write_str(".bind(client, ").unwrap();
                        {
                            let iter = params_name.into_iter();
                            let sep = ',';
                            let first = true;
                            for (params_name,) in iter {}
                        }
                        w.write_str(") ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&post)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" }
}").unwrap();
                    };
                }
            }
        }
    }
    /// Generates type definitions for custom user types. This includes domains, composites and enums.
    /// If the type is not `Copy`, then a Borrowed version will be generated.
    fn gen_custom_type(
        w: &mut impl Write,
        schema: &str,
        prepared: &PreparedType,
        CodegenSettings { derive_ser, is_async }: CodegenSettings,
    ) {
        let PreparedType { struct_name, content, is_copy, is_params, name } = prepared;
        let copy = if *is_copy { "Copy," } else { "" };
        let ser_str = if derive_ser { "serde::Serialize," } else { "" };
        let name_str = {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["\"", "\""],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                ),
            );
            res
        };
        match content {
            PreparedContent::Enum(variants) => {
                {
                    w.write_str("#[derive(").unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&ser_str)],
                            ),
                        )
                        .unwrap();
                    w.write_str(
                            " Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)] pub enum ",
                        )
                        .unwrap();
                    w.write_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                            ),
                        )
                        .unwrap();
                    w.write_str(" { ").unwrap();
                    {
                        let iter = variants.into_iter();
                        let sep = ',';
                        let first = true;
                        for (variants,) in iter {}
                    }
                    w.write_str(" }").unwrap();
                };
                enum_sql(w, name, struct_name, variants);
            }
            PreparedContent::Composite(fields) => {
                let fields_name = fields.iter().map(|p| &p.name);
                {
                    let fields_name = fields_name.clone();
                    let fields_ty = fields.iter().map(|p| p.own_struct());
                    {
                        w.write_str("#[derive(").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&ser_str)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" Debug, postgres_types :: FromSql, ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&copy)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" Clone, PartialEq)]
#[postgres(name = ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&name_str)],
                                ),
                            )
                            .unwrap();
                        w.write_str(")] pub struct ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
{ ").unwrap();
                        {
                            let iter = fields_name
                                .into_iter()
                                .zip(fields_ty.into_iter());
                            let sep = ',';
                            let first = true;
                            for (fields_name, fields_ty) in iter {}
                        }
                        w.write_str(" }").unwrap();
                    };
                }
                if *is_copy {
                    struct_tosql(
                        w,
                        struct_name,
                        fields,
                        name,
                        false,
                        *is_params,
                        is_async,
                    );
                } else {
                    let fields_owning = fields.iter().map(|p| p.owning_assign());
                    let fields_name2 = fields_name.clone();
                    let fields_name3 = fields_name.clone();
                    let fields_brw = fields.iter().map(|p| p.brw_ty(true, is_async));
                    let brw_name = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", "Borrowed"],
                                &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                            ),
                        );
                        res
                    };
                    {
                        w.write_str("#[derive(Debug)] pub struct ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < 'a >
{ ").unwrap();
                        {
                            let iter = fields_name2
                                .into_iter()
                                .zip(fields_brw.into_iter());
                            let sep = ',';
                            let first = true;
                            for (fields_name2, fields_brw) in iter {}
                        }
                        w.write_str(" } impl < 'a > From < ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" <
'a >> for ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
{
    fn from(").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" { ").unwrap();
                        {
                            let iter = fields_name3.into_iter();
                            let sep = ',';
                            let first = true;
                            for (fields_name3,) in iter {}
                        }
                        w.write_str(" } : ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&brw_name)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" < 'a >,) -> Self
    { Self { ").unwrap();
                        {
                            let iter = fields_owning.into_iter();
                            let sep = ',';
                            let first = true;
                            for (fields_owning,) in iter {}
                        }
                        w.write_str(" } }
}").unwrap();
                    };
                    composite_fromsql(w, struct_name, fields, name, schema);
                    if !is_params {
                        let param_name = {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "Params"],
                                    &[::core::fmt::ArgumentV1::new_display(&struct_name)],
                                ),
                            );
                            res
                        };
                        let fields_ty = fields.iter().map(|p| p.param_ty(is_async));
                        let derive = if *is_copy { ",Copy,Clone" } else { "" };
                        {
                            w.write_str("#[derive(Debug ").unwrap();
                            w.write_fmt(
                                    ::core::fmt::Arguments::new_v1(
                                        &[""],
                                        &[::core::fmt::ArgumentV1::new_display(&derive)],
                                    ),
                                )
                                .unwrap();
                            w.write_str(")] pub struct ").unwrap();
                            w.write_fmt(
                                    ::core::fmt::Arguments::new_v1(
                                        &[""],
                                        &[::core::fmt::ArgumentV1::new_display(&param_name)],
                                    ),
                                )
                                .unwrap();
                            w.write_str(" < 'a >
{ ").unwrap();
                            {
                                let iter = fields_name
                                    .into_iter()
                                    .zip(fields_ty.into_iter());
                                let sep = ',';
                                let first = true;
                                for (fields_name, fields_ty) in iter {}
                            }
                            w.write_str(" }").unwrap();
                        };
                    }
                    struct_tosql(
                        w,
                        struct_name,
                        fields,
                        name,
                        true,
                        *is_params,
                        is_async,
                    );
                }
            }
        }
    }
    fn gen_type_modules(
        w: &mut impl Write,
        prepared: &IndexMap<String, Vec<PreparedType>>,
        settings: CodegenSettings,
    ) {
        let modules = prepared
            .iter()
            .map(|(schema, types)| {
                Lazy::new(move |w| {
                    let lazy = Lazy::new(|w| {
                        for ty in types {
                            gen_custom_type(w, schema, ty, settings)
                        }
                    });
                    {
                        w.write_str("pub mod ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&schema)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" { ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&lazy)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" }").unwrap();
                    };
                })
            });
        {
            w.write_str(
                    "#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod types { ",
                )
                .unwrap();
            {
                let iter = modules.into_iter();
                let sep = '*';
                let first = true;
                for (modules,) in iter {}
            }
            w.write_str(" }").unwrap();
        };
    }
    pub(crate) fn generate(
        preparation: Preparation,
        settings: CodegenSettings,
    ) -> String {
        let import = if settings.is_async {
            "use futures::{{StreamExt, TryStreamExt}};use futures; use cornucopia_async::GenericClient;"
        } else {
            "use postgres::{{fallible_iterator::FallibleIterator,GenericClient}};"
        };
        let mut buff = "// This file was generated with `cornucopia`. Do not modify.\n\n"
            .to_string();
        let w = &mut buff;
        gen_type_modules(w, &preparation.types, settings);
        let query_modules = preparation
            .modules
            .iter()
            .map(|module| {
                Lazy::new(move |w| {
                    let name = &module.info.name;
                    let params_string = module
                        .params
                        .values()
                        .map(|params| Lazy::new(|w| gen_params_struct(
                            w,
                            params,
                            settings,
                        )));
                    let rows_string = module
                        .rows
                        .values()
                        .map(|row| Lazy::new(|w| gen_row_structs(w, row, settings)));
                    let queries_string = module
                        .queries
                        .values()
                        .map(|query| Lazy::new(|w| gen_query_fn(
                            w,
                            module,
                            query,
                            settings,
                        )));
                    {
                        w.write_str("pub mod ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&name)],
                                ),
                            )
                            .unwrap();
                        w.write_str("
{ ").unwrap();
                        w.write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[::core::fmt::ArgumentV1::new_display(&import)],
                                ),
                            )
                            .unwrap();
                        w.write_str(" ").unwrap();
                        {
                            let iter = params_string.into_iter();
                            let sep = '*';
                            let first = true;
                            for (params_string,) in iter {}
                        }
                        w.write_str(" ").unwrap();
                        {
                            let iter = rows_string.into_iter();
                            let sep = '*';
                            let first = true;
                            for (rows_string,) in iter {}
                        }
                        w.write_str(" ").unwrap();
                        {
                            let iter = queries_string.into_iter();
                            let sep = '*';
                            let first = true;
                            for (queries_string,) in iter {}
                        }
                        w.write_str(" }").unwrap();
                    };
                })
            });
        {
            w.write_str(
                    "#[allow(clippy :: all, clippy :: pedantic)] #[allow(unused_variables)]
#[allow(unused_imports)] #[allow(dead_code)] pub mod queries
{ ",
                )
                .unwrap();
            {
                let iter = query_modules.into_iter();
                let sep = '*';
                let first = true;
                for (query_modules,) in iter {}
            }
            w.write_str(" }").unwrap();
        };
        buff
    }
}
mod error {
    use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
    use thiserror::Error as ThisError;
    /// Enumeration of all the errors reported by Cornucopia.
    #[error(transparent)]
    #[diagnostic(transparent)]
    pub enum Error {
        /// An error while trying to connect to a database.
        Connection(#[from] crate::conn::error::Error),
        /// An error while trying to read PostgreSQL query files.
        ReadQueries(#[from] crate::read_queries::error::Error),
        /// An error while trying to parse PostgreSQL query files.
        ParseQueries(#[from] crate::parser::error::Error),
        /// An error while trying to validate PostgreSQL query files.
        ValidateQueries(#[from] crate::validation::error::Error),
        /// An error while manipulating a container managed by Cornucopia.
        Container(#[from] crate::container::error::Error),
        /// An error while trying to prepare PostgreSQL queries.
        PrepareQueries(#[from] crate::prepare_queries::error::Error),
        /// An error while reading PostgreSQL schema files.
        LoadSchema(#[from] crate::load_schema::error::Error),
        /// An error while trying to write the generated code to its destination file.
        WriteCodeGenFile(#[from] WriteOutputError),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Error {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Error::Connection(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Connection",
                        &__self_0,
                    )
                }
                Error::ReadQueries(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ReadQueries",
                        &__self_0,
                    )
                }
                Error::ParseQueries(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ParseQueries",
                        &__self_0,
                    )
                }
                Error::ValidateQueries(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "ValidateQueries",
                        &__self_0,
                    )
                }
                Error::Container(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Container",
                        &__self_0,
                    )
                }
                Error::PrepareQueries(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PrepareQueries",
                        &__self_0,
                    )
                }
                Error::LoadSchema(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "LoadSchema",
                        &__self_0,
                    )
                }
                Error::WriteCodeGenFile(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "WriteCodeGenFile",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for Error {
        fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
            use thiserror::__private::AsDynError;
            #[allow(deprecated)]
            match self {
                Error::Connection { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::ReadQueries { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::ParseQueries { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::ValidateQueries { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::Container { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::PrepareQueries { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::LoadSchema { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
                Error::WriteCodeGenFile { 0: transparent } => {
                    std::error::Error::source(transparent.as_dyn_error())
                }
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::fmt::Display for Error {
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
            match self {
                Error::Connection(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::ReadQueries(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::ParseQueries(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::ValidateQueries(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::Container(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::PrepareQueries(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::LoadSchema(_0) => std::fmt::Display::fmt(_0, __formatter),
                Error::WriteCodeGenFile(_0) => std::fmt::Display::fmt(_0, __formatter),
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::conn::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::conn::error::Error) -> Self {
            Error::Connection { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::read_queries::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::read_queries::error::Error) -> Self {
            Error::ReadQueries { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::parser::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::parser::error::Error) -> Self {
            Error::ParseQueries { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::validation::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::validation::error::Error) -> Self {
            Error::ValidateQueries {
                0: source,
            }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::container::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::container::error::Error) -> Self {
            Error::Container { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::prepare_queries::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::prepare_queries::error::Error) -> Self {
            Error::PrepareQueries { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<crate::load_schema::error::Error> for Error {
        #[allow(deprecated)]
        fn from(source: crate::load_schema::error::Error) -> Self {
            Error::LoadSchema { 0: source }
        }
    }
    #[allow(unused_qualifications)]
    impl std::convert::From<WriteOutputError> for Error {
        #[allow(deprecated)]
        fn from(source: WriteOutputError) -> Self {
            Error::WriteCodeGenFile {
                0: source,
            }
        }
    }
    impl miette::Diagnostic for Error {
        fn code<'a>(
            &'a self,
        ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.code(),
                Self::ReadQueries(unnamed, ..) => unnamed.code(),
                Self::ParseQueries(unnamed, ..) => unnamed.code(),
                Self::ValidateQueries(unnamed, ..) => unnamed.code(),
                Self::Container(unnamed, ..) => unnamed.code(),
                Self::PrepareQueries(unnamed, ..) => unnamed.code(),
                Self::LoadSchema(unnamed, ..) => unnamed.code(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.code(),
                _ => std::option::Option::None,
            }
        }
        fn help<'a>(
            &'a self,
        ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.help(),
                Self::ReadQueries(unnamed, ..) => unnamed.help(),
                Self::ParseQueries(unnamed, ..) => unnamed.help(),
                Self::ValidateQueries(unnamed, ..) => unnamed.help(),
                Self::Container(unnamed, ..) => unnamed.help(),
                Self::PrepareQueries(unnamed, ..) => unnamed.help(),
                Self::LoadSchema(unnamed, ..) => unnamed.help(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.help(),
                _ => std::option::Option::None,
            }
        }
        fn severity(&self) -> std::option::Option<miette::Severity> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.severity(),
                Self::ReadQueries(unnamed, ..) => unnamed.severity(),
                Self::ParseQueries(unnamed, ..) => unnamed.severity(),
                Self::ValidateQueries(unnamed, ..) => unnamed.severity(),
                Self::Container(unnamed, ..) => unnamed.severity(),
                Self::PrepareQueries(unnamed, ..) => unnamed.severity(),
                Self::LoadSchema(unnamed, ..) => unnamed.severity(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.severity(),
                _ => std::option::Option::None,
            }
        }
        fn labels(
            &self,
        ) -> std::option::Option<
            std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
        > {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.labels(),
                Self::ReadQueries(unnamed, ..) => unnamed.labels(),
                Self::ParseQueries(unnamed, ..) => unnamed.labels(),
                Self::ValidateQueries(unnamed, ..) => unnamed.labels(),
                Self::Container(unnamed, ..) => unnamed.labels(),
                Self::PrepareQueries(unnamed, ..) => unnamed.labels(),
                Self::LoadSchema(unnamed, ..) => unnamed.labels(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.labels(),
                _ => std::option::Option::None,
            }
        }
        fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.source_code(),
                Self::ReadQueries(unnamed, ..) => unnamed.source_code(),
                Self::ParseQueries(unnamed, ..) => unnamed.source_code(),
                Self::ValidateQueries(unnamed, ..) => unnamed.source_code(),
                Self::Container(unnamed, ..) => unnamed.source_code(),
                Self::PrepareQueries(unnamed, ..) => unnamed.source_code(),
                Self::LoadSchema(unnamed, ..) => unnamed.source_code(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.source_code(),
                _ => std::option::Option::None,
            }
        }
        fn related(
            &self,
        ) -> std::option::Option<
            std::boxed::Box<dyn std::iter::Iterator<Item = &dyn miette::Diagnostic> + '_>,
        > {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.related(),
                Self::ReadQueries(unnamed, ..) => unnamed.related(),
                Self::ParseQueries(unnamed, ..) => unnamed.related(),
                Self::ValidateQueries(unnamed, ..) => unnamed.related(),
                Self::Container(unnamed, ..) => unnamed.related(),
                Self::PrepareQueries(unnamed, ..) => unnamed.related(),
                Self::LoadSchema(unnamed, ..) => unnamed.related(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.related(),
                _ => std::option::Option::None,
            }
        }
        fn url<'a>(
            &'a self,
        ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.url(),
                Self::ReadQueries(unnamed, ..) => unnamed.url(),
                Self::ParseQueries(unnamed, ..) => unnamed.url(),
                Self::ValidateQueries(unnamed, ..) => unnamed.url(),
                Self::Container(unnamed, ..) => unnamed.url(),
                Self::PrepareQueries(unnamed, ..) => unnamed.url(),
                Self::LoadSchema(unnamed, ..) => unnamed.url(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.url(),
                _ => std::option::Option::None,
            }
        }
        fn diagnostic_source(&self) -> std::option::Option<&dyn miette::Diagnostic> {
            #[allow(unused_variables, deprecated)]
            match self {
                Self::Connection(unnamed, ..) => unnamed.diagnostic_source(),
                Self::ReadQueries(unnamed, ..) => unnamed.diagnostic_source(),
                Self::ParseQueries(unnamed, ..) => unnamed.diagnostic_source(),
                Self::ValidateQueries(unnamed, ..) => unnamed.diagnostic_source(),
                Self::Container(unnamed, ..) => unnamed.diagnostic_source(),
                Self::PrepareQueries(unnamed, ..) => unnamed.diagnostic_source(),
                Self::LoadSchema(unnamed, ..) => unnamed.diagnostic_source(),
                Self::WriteCodeGenFile(unnamed, ..) => unnamed.diagnostic_source(),
                _ => std::option::Option::None,
            }
        }
    }
    impl Error {
        #[must_use]
        pub fn report(self) -> String {
            let mut buff = String::new();
            GraphicalReportHandler::new()
                .with_theme(GraphicalTheme::unicode_nocolor())
                .render_report(&mut buff, &self)
                .unwrap();
            buff
        }
    }
    #[error("Could not write your queries to destination file `{file_path}`: ({err})")]
    pub struct WriteOutputError {
        pub(crate) file_path: String,
        pub(crate) err: std::io::Error,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for WriteOutputError {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "WriteOutputError",
                "file_path",
                &&self.file_path,
                "err",
                &&self.err,
            )
        }
    }
    #[allow(unused_qualifications)]
    impl std::error::Error for WriteOutputError {}
    #[allow(unused_qualifications)]
    impl std::fmt::Display for WriteOutputError {
        #[allow(clippy::used_underscore_binding)]
        fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            #[allow(unused_imports)]
            use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
            #[allow(unused_variables, deprecated)]
            let Self { file_path, err } = self;
            __formatter
                .write_fmt(
                    ::core::fmt::Arguments::new_v1(
                        &[
                            "Could not write your queries to destination file `",
                            "`: (",
                            ")",
                        ],
                        &[
                            ::core::fmt::ArgumentV1::new_display(
                                &file_path.as_display(),
                            ),
                            ::core::fmt::ArgumentV1::new_display(&err.as_display()),
                        ],
                    ),
                )
        }
    }
    impl miette::Diagnostic for WriteOutputError {}
}
mod load_schema {
    use miette::NamedSource;
    use postgres::Client;
    use crate::utils::db_err;
    use self::error::Error;
    /// Loads PostgreSQL schemas into a database.
    ///
    /// Takes a list of file paths as parameter and loads them in their given order.
    pub fn load_schema(client: &mut Client, paths: Vec<String>) -> Result<(), Error> {
        for path in paths {
            let sql = std::fs::read_to_string(&path)
                .map_err(|err| Error::Io {
                    path: path.clone(),
                    err,
                })?;
            client
                .batch_execute(&sql)
                .map_err(|err| {
                    let msg = {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&err)],
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                        res
                    };
                    let src = NamedSource::new(path, sql);
                    if let Some((position, msg, help)) = db_err(&err) {
                        Error::Postgres {
                            msg,
                            help,
                            src,
                            err_span: Some((position as usize..position as usize).into()),
                        }
                    } else {
                        Error::Postgres {
                            msg,
                            help: None,
                            src,
                            err_span: None,
                        }
                    }
                })?;
        }
        Ok(())
    }
    pub(crate) mod error {
        use miette::{Diagnostic, NamedSource, SourceSpan};
        use thiserror::Error as ThisError;
        pub enum Error {
            #[error("Could not read schema `{path}`: ({err})")]
            Io { path: String, err: std::io::Error },
            #[error("Could not execute schema: {msg}")]
            Postgres {
                msg: String,
                #[source_code]
                src: NamedSource,
                #[help]
                help: Option<String>,
                #[label("error occurs near this location")]
                err_span: Option<SourceSpan>,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::Io { path: __self_0, err: __self_1 } => {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "Io",
                            "path",
                            &__self_0,
                            "err",
                            &__self_1,
                        )
                    }
                    Error::Postgres {
                        msg: __self_0,
                        src: __self_1,
                        help: __self_2,
                        err_span: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "Postgres",
                            "msg",
                            &__self_0,
                            "src",
                            &__self_1,
                            "help",
                            &__self_2,
                            "err_span",
                            &__self_3,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::Io { path, err } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["Could not read schema `", "`: (", ")"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&path.as_display()),
                                        ::core::fmt::ArgumentV1::new_display(&err.as_display()),
                                    ],
                                ),
                            )
                    }
                    Error::Postgres { msg, src, help, err_span } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["Could not execute schema: "],
                                    &[::core::fmt::ArgumentV1::new_display(&msg.as_display())],
                                ),
                            )
                    }
                }
            }
        }
        impl miette::Diagnostic for Error {
            fn help<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Postgres { msg, src, help, err_span } => {
                        use miette::macro_helpers::ToOption;
                        miette::macro_helpers::OptionalWrapper::<Option<String>>::new()
                            .to_option(&help)
                            .as_ref()
                            .map(|
                                __miette_internal_var,
                            | -> std::boxed::Box<dyn std::fmt::Display + 'a> {
                                std::boxed::Box::new({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&__miette_internal_var),
                                            ],
                                        ),
                                    );
                                    res
                                })
                            })
                    }
                    _ => std::option::Option::None,
                }
            }
            fn labels(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
            > {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Postgres { msg, src, help, err_span } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<
                                                Option<SourceSpan>,
                                            >::new()
                                                .to_option(err_span)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["error occurs near this location"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    _ => std::option::Option::None,
                }
            }
            fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Postgres { msg, src, help, err_span } => {
                        std::option::Option::Some(src)
                    }
                    _ => std::option::Option::None,
                }
            }
        }
    }
}
mod parser {
    use std::{fmt::Display, ops::Range};
    use chumsky::prelude::*;
    use error::Error;
    use heck::ToUpperCamelCase;
    use miette::SourceSpan;
    use crate::read_queries::ModuleInfo;
    /// Th    if is data structure holds a value and the context in which it was parsed.
    /// This context is used for error reporting.
    pub struct Span<T> {
        pub(crate) span: SourceSpan,
        pub(crate) value: T,
    }
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for Span<T> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Span",
                "span",
                &&self.span,
                "value",
                &&self.value,
            )
        }
    }
    #[automatically_derived]
    impl<T: ::core::clone::Clone> ::core::clone::Clone for Span<T> {
        #[inline]
        fn clone(&self) -> Span<T> {
            Span {
                span: ::core::clone::Clone::clone(&self.span),
                value: ::core::clone::Clone::clone(&self.value),
            }
        }
    }
    impl<T: std::hash::Hash> std::hash::Hash for Span<T> {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.value.hash(state);
        }
    }
    impl<T: PartialEq> PartialEq<Self> for Span<T> {
        fn eq(&self, other: &Self) -> bool {
            self.value == other.value
        }
    }
    impl<T: Display> Display for Span<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.value.fmt(f)
        }
    }
    impl<T: Eq> Eq for Span<T> {}
    impl<T: PartialOrd + PartialEq> PartialOrd<Self> for Span<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.value.partial_cmp(&other.value)
        }
    }
    impl<T: Ord> Ord for Span<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }
    impl<T> Span<T> {
        pub(crate) fn map<U>(&self, f: impl Fn(&T) -> U) -> Span<U> {
            Span {
                value: f(&self.value),
                span: self.span,
            }
        }
    }
    fn ident() -> impl Parser<char, Span<String>, Error = Simple<char>> {
        filter(char::is_ascii_alphabetic)
            .chain(filter(|c: &char| c.is_ascii_alphanumeric() || *c == '_').repeated())
            .collect()
            .map_with_span(|value: String, span: Range<usize>| Span {
                value,
                span: span.into(),
            })
    }
    fn ln() -> impl Parser<char, (), Error = Simple<char>> {
        just("\n").or(just("\n\r")).ignored()
    }
    fn space() -> impl Parser<char, (), Error = Simple<char>> {
        filter(|c: &char| c.is_whitespace() && *c != '\n').repeated().ignored()
    }
    fn blank() -> impl Parser<char, (), Error = Simple<char>> {
        let comment = just("--")
            .then(none_of(":!").rewind())
            .then(none_of('\n').repeated());
        filter(|c: &char| c.is_whitespace())
            .ignored()
            .or(comment.ignored())
            .repeated()
            .ignored()
    }
    pub struct NullableIdent {
        pub name: Span<String>,
        pub nullable: bool,
        pub inner_nullable: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for NullableIdent {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "NullableIdent",
                "name",
                &&self.name,
                "nullable",
                &&self.nullable,
                "inner_nullable",
                &&self.inner_nullable,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for NullableIdent {
        #[inline]
        fn clone(&self) -> NullableIdent {
            NullableIdent {
                name: ::core::clone::Clone::clone(&self.name),
                nullable: ::core::clone::Clone::clone(&self.nullable),
                inner_nullable: ::core::clone::Clone::clone(&self.inner_nullable),
            }
        }
    }
    fn parse_nullable_ident() -> impl Parser<
        char,
        Vec<NullableIdent>,
        Error = Simple<char>,
    > {
        space()
            .ignore_then(ident())
            .then(just('?').or_not())
            .then(just("[?]").or_not())
            .map(|((name, null), inner_null)| NullableIdent {
                name,
                nullable: null.is_some(),
                inner_nullable: inner_null.is_some(),
            })
            .then_ignore(space())
            .separated_by(just(','))
            .allow_trailing()
            .delimited_by(just('('), just(')'))
    }
    pub struct TypeAnnotation {
        pub name: Span<String>,
        pub fields: Vec<NullableIdent>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeAnnotation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "TypeAnnotation",
                "name",
                &&self.name,
                "fields",
                &&self.fields,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TypeAnnotation {
        #[inline]
        fn clone(&self) -> TypeAnnotation {
            TypeAnnotation {
                name: ::core::clone::Clone::clone(&self.name),
                fields: ::core::clone::Clone::clone(&self.fields),
            }
        }
    }
    impl TypeAnnotation {
        fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
            just("--:")
                .ignore_then(space())
                .ignore_then(ident())
                .then_ignore(space())
                .then(parse_nullable_ident())
                .map(|(name, fields)| Self { name, fields })
        }
    }
    pub(crate) struct Query {
        pub(crate) name: Span<String>,
        pub(crate) param: QueryDataStruct,
        pub(crate) row: QueryDataStruct,
        pub(crate) sql_span: SourceSpan,
        pub(crate) sql_str: String,
        pub(crate) bind_params: Vec<Span<String>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Query {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "name",
                "param",
                "row",
                "sql_span",
                "sql_str",
                "bind_params",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &&self.name,
                &&self.param,
                &&self.row,
                &&self.sql_span,
                &&self.sql_str,
                &&self.bind_params,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "Query", names, values)
        }
    }
    impl Query {
        /// Escape sql string and pattern that are not bind
        fn sql_escaping() -> impl Parser<char, (), Error = Simple<char>> {
            let cast = just("::").ignored();
            let constant = none_of("\"")
                .repeated()
                .delimited_by(just("\""), just("\""))
                .ignored();
            let string = none_of("'")
                .repeated()
                .delimited_by(just("'"), just("'"))
                .ignored();
            let c_style_string = just("\\'")
                .or(just("''"))
                .ignored()
                .or(none_of("'").ignored())
                .repeated()
                .delimited_by(just("e'").or(just("E'")), just("'"))
                .ignored();
            let dollar_tag = just("$").then(none_of("$").repeated()).then(just("$"));
            let dollar_quoted = none_of("$")
                .repeated()
                .delimited_by(dollar_tag.clone(), dollar_tag)
                .ignored();
            c_style_string
                .or(cast)
                .or(string)
                .or(constant)
                .or(dollar_quoted)
                .or(one_of("eE").then(none_of("'").rewind()).ignored())
                .or(none_of("\"':$eE").ignored())
                .repeated()
                .at_least(1)
                .ignored()
        }
        /// Parse all bind from an SQL query
        fn parse_bind() -> impl Parser<char, Vec<Span<String>>, Error = Simple<char>> {
            just(':')
                .ignore_then(ident())
                .separated_by(Self::sql_escaping())
                .allow_leading()
                .allow_trailing()
        }
        /// Parse sql query, normalizing named parameters
        fn parse_sql_query() -> impl Parser<
            char,
            (String, SourceSpan, Vec<Span<String>>),
            Error = Simple<char>,
        > {
            none_of(";")
                .repeated()
                .then_ignore(just(';'))
                .collect::<String>()
                .map_with_span(|mut sql_str, span: Range<usize>| {
                    let bind_params: Vec<_> = Self::parse_bind()
                        .parse(sql_str.clone())
                        .unwrap();
                    let dedup_params: Vec<_> = bind_params
                        .iter()
                        .enumerate()
                        .rev()
                        .filter_map(|(i, u)| {
                            (!bind_params[..i].contains(u)).then(|| u.clone())
                        })
                        .rev()
                        .collect();
                    for bind_param in bind_params.iter().rev() {
                        let index = dedup_params
                            .iter()
                            .position(|bp| bp == bind_param)
                            .unwrap();
                        let start = bind_param.span.offset() - 1;
                        let end = start + bind_param.span.len();
                        sql_str
                            .replace_range(
                                start..=end,
                                &{
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["$"],
                                            &[::core::fmt::ArgumentV1::new_display(&(index + 1))],
                                        ),
                                    );
                                    res
                                },
                            );
                    }
                    (sql_str, span.into(), dedup_params)
                })
        }
        fn parse_query_annotation() -> impl Parser<
            char,
            (Span<String>, QueryDataStruct, QueryDataStruct),
            Error = Simple<char>,
        > {
            just("--!")
                .ignore_then(space())
                .ignore_then(ident())
                .then_ignore(space())
                .then(QueryDataStruct::parser())
                .then_ignore(space())
                .then(
                    just(':')
                        .ignore_then(space())
                        .ignore_then(QueryDataStruct::parser())
                        .or_not(),
                )
                .map(|((name, param), row)| (name, param, row.unwrap_or_default()))
        }
        fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
            Self::parse_query_annotation()
                .then_ignore(space())
                .then_ignore(ln())
                .then(Self::parse_sql_query())
                .map(|((name, param, row), (sql_str, sql_span, bind_params))| Self {
                    name,
                    param,
                    row,
                    sql_span,
                    sql_str,
                    bind_params,
                })
        }
    }
    pub(crate) struct QueryDataStruct {
        pub span: SourceSpan,
        pub name: Option<Span<String>>,
        pub idents: Option<Vec<NullableIdent>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for QueryDataStruct {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "QueryDataStruct",
                "span",
                &&self.span,
                "name",
                &&self.name,
                "idents",
                &&self.idents,
            )
        }
    }
    impl QueryDataStruct {
        pub fn is_implicit(&self) -> bool {
            self.name.is_none()
        }
        pub fn is_empty(&self) -> bool {
            self.name.is_none() && self.idents.is_none()
        }
        pub fn inlined(&self) -> bool {
            self.idents.is_some() && self.name.is_some()
        }
        pub(crate) fn name_and_fields<'a>(
            &'a self,
            registered_structs: &'a [TypeAnnotation],
            query_name: &Span<String>,
            name_suffix: Option<&str>,
        ) -> (&'a [NullableIdent], Span<String>) {
            if let Some(named) = &self.name {
                (
                    self
                        .idents
                        .as_ref()
                        .map_or_else(
                            || {
                                registered_structs
                                    .iter()
                                    .find_map(|it| {
                                        (it.name == *named).then_some(it.fields.as_slice())
                                    })
                                    .unwrap_or(&[])
                            },
                            Vec::as_slice,
                        ),
                    named.clone(),
                )
            } else {
                (
                    self.idents.as_ref().map_or(&[], Vec::as_slice),
                    query_name
                        .map(|x| {
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["", ""],
                                        &[
                                            ::core::fmt::ArgumentV1::new_display(
                                                &x.to_upper_camel_case(),
                                            ),
                                            ::core::fmt::ArgumentV1::new_display(
                                                &name_suffix.unwrap_or_default(),
                                            ),
                                        ],
                                    ),
                                );
                                res
                            }
                        }),
                )
            }
        }
    }
    impl Default for QueryDataStruct {
        fn default() -> Self {
            Self {
                span: (0..0).into(),
                name: None,
                idents: None,
            }
        }
    }
    impl QueryDataStruct {
        fn parser() -> impl Parser<char, Self, Error = Simple<char>> {
            ident()
                .or_not()
                .then_ignore(space())
                .then(parse_nullable_ident().or_not())
                .map_with_span(|(name, idents), span| Self {
                    span: span.into(),
                    name,
                    idents,
                })
        }
    }
    enum Statement {
        Type(TypeAnnotation),
        Query(Query),
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Statement {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                Statement::Type(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Type",
                        &__self_0,
                    )
                }
                Statement::Query(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Query",
                        &__self_0,
                    )
                }
            }
        }
    }
    pub(crate) struct Module {
        pub(crate) info: ModuleInfo,
        pub(crate) types: Vec<TypeAnnotation>,
        pub(crate) queries: Vec<Query>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Module {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "Module",
                "info",
                &&self.info,
                "types",
                &&self.types,
                "queries",
                &&self.queries,
            )
        }
    }
    pub(crate) fn parse_query_module(info: ModuleInfo) -> Result<Module, Error> {
        match TypeAnnotation::parser()
            .map(Statement::Type)
            .or(Query::parser().map(Statement::Query))
            .separated_by(blank())
            .allow_leading()
            .allow_trailing()
            .then_ignore(end())
            .parse(info.content.as_str())
        {
            Ok(statements) => {
                let mut types = Vec::new();
                let mut queries = Vec::new();
                for item in statements {
                    match item {
                        Statement::Type(it) => types.push(it),
                        Statement::Query(it) => queries.push(it),
                    }
                }
                Ok(Module { info, types, queries })
            }
            Err(e) => {
                Err(Error {
                    src: info.into(),
                    err_span: e[0].span().into(),
                    help: e[0].to_string().replace('\n', "\\n"),
                })
            }
        }
    }
    pub(crate) mod error {
        use miette::{Diagnostic, NamedSource, SourceSpan};
        use thiserror::Error as ThisError;
        #[error("Couldn't parse queries")]
        pub struct Error {
            #[source_code]
            pub src: NamedSource,
            #[help]
            pub help: String,
            #[label("unexpected token")]
            pub err_span: SourceSpan,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Error",
                    "src",
                    &&self.src,
                    "help",
                    &&self.help,
                    "err_span",
                    &&self.err_span,
                )
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_variables, deprecated)]
                let Self { src, help, err_span } = self;
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(&["Couldn\'t parse queries"], &[]),
                    )
            }
        }
        impl miette::Diagnostic for Error {
            fn help<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                let Self { src, help, err_span } = self;
                use miette::macro_helpers::ToOption;
                miette::macro_helpers::OptionalWrapper::<String>::new()
                    .to_option(&self.help)
                    .as_ref()
                    .map(|
                        __miette_internal_var,
                    | -> std::boxed::Box<dyn std::fmt::Display + 'a> {
                        std::boxed::Box::new({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&__miette_internal_var),
                                    ],
                                ),
                            );
                            res
                        })
                    })
            }
            #[allow(unused_variables)]
            fn labels(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
            > {
                use miette::macro_helpers::ToOption;
                let Self { src, help, err_span } = self;
                std::option::Option::Some(
                    Box::new(
                        <[_]>::into_vec(
                                #[rustc_box]
                                ::alloc::boxed::Box::new([
                                    miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                        .to_option(&self.err_span)
                                        .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                            std::option::Option::Some({
                                                let res = ::alloc::fmt::format(
                                                    ::core::fmt::Arguments::new_v1(&["unexpected token"], &[]),
                                                );
                                                res
                                            }),
                                            __miette_internal_var.clone(),
                                        )),
                                ]),
                            )
                            .into_iter()
                            .filter(Option::is_some)
                            .map(Option::unwrap),
                    ),
                )
            }
            #[allow(unused_variables)]
            fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
                let Self { src, help, err_span } = self;
                Some(&self.src)
            }
        }
    }
}
mod prepare_queries {
    use std::rc::Rc;
    use heck::ToUpperCamelCase;
    use indexmap::{map::Entry, IndexMap};
    use postgres::Client;
    use postgres_types::{Kind, Type};
    use crate::{
        parser::{Module, NullableIdent, Query, Span, TypeAnnotation},
        read_queries::ModuleInfo, type_registrar::CornucopiaType,
        type_registrar::TypeRegistrar, utils::escape_keyword, validation,
    };
    use self::error::Error;
    /// This data structure is used by Cornucopia to generate
    /// all constructs related to this particular query.
    pub(crate) struct PreparedQuery {
        pub(crate) name: String,
        pub(crate) param: Option<(usize, Vec<usize>)>,
        pub(crate) row: Option<(usize, Vec<usize>)>,
        pub(crate) sql: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedQuery {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "PreparedQuery",
                "name",
                &&self.name,
                "param",
                &&self.param,
                "row",
                &&self.row,
                "sql",
                &&self.sql,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedQuery {
        #[inline]
        fn clone(&self) -> PreparedQuery {
            PreparedQuery {
                name: ::core::clone::Clone::clone(&self.name),
                param: ::core::clone::Clone::clone(&self.param),
                row: ::core::clone::Clone::clone(&self.row),
                sql: ::core::clone::Clone::clone(&self.sql),
            }
        }
    }
    /// A row or params field
    pub struct PreparedField {
        pub(crate) name: String,
        pub(crate) ty: Rc<CornucopiaType>,
        pub(crate) is_nullable: bool,
        pub(crate) is_inner_nullable: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedField {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "PreparedField",
                "name",
                &&self.name,
                "ty",
                &&self.ty,
                "is_nullable",
                &&self.is_nullable,
                "is_inner_nullable",
                &&self.is_inner_nullable,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedField {
        #[inline]
        fn clone(&self) -> PreparedField {
            PreparedField {
                name: ::core::clone::Clone::clone(&self.name),
                ty: ::core::clone::Clone::clone(&self.ty),
                is_nullable: ::core::clone::Clone::clone(&self.is_nullable),
                is_inner_nullable: ::core::clone::Clone::clone(&self.is_inner_nullable),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PreparedField {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PreparedField {
        #[inline]
        fn eq(&self, other: &PreparedField) -> bool {
            self.name == other.name && self.ty == other.ty
                && self.is_nullable == other.is_nullable
                && self.is_inner_nullable == other.is_inner_nullable
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PreparedField {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PreparedField {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<Rc<CornucopiaType>>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    impl PreparedField {
        pub(crate) fn new(
            name: String,
            ty: Rc<CornucopiaType>,
            nullity: Option<&NullableIdent>,
        ) -> Self {
            Self {
                name: escape_keyword(name),
                ty,
                is_nullable: nullity.map_or(false, |it| it.nullable),
                is_inner_nullable: nullity.map_or(false, |it| it.inner_nullable),
            }
        }
    }
    impl PreparedField {
        pub fn unwrapped_name(&self) -> String {
            self.own_struct().replace(['<', '>', '_'], "").to_upper_camel_case()
        }
    }
    pub(crate) struct PreparedItem {
        pub(crate) name: Span<String>,
        pub(crate) fields: Vec<PreparedField>,
        pub(crate) is_copy: bool,
        pub(crate) is_named: bool,
        pub(crate) is_ref: bool,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PreparedItem",
                "name",
                &&self.name,
                "fields",
                &&self.fields,
                "is_copy",
                &&self.is_copy,
                "is_named",
                &&self.is_named,
                "is_ref",
                &&self.is_ref,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedItem {
        #[inline]
        fn clone(&self) -> PreparedItem {
            PreparedItem {
                name: ::core::clone::Clone::clone(&self.name),
                fields: ::core::clone::Clone::clone(&self.fields),
                is_copy: ::core::clone::Clone::clone(&self.is_copy),
                is_named: ::core::clone::Clone::clone(&self.is_named),
                is_ref: ::core::clone::Clone::clone(&self.is_ref),
            }
        }
    }
    impl PreparedItem {
        pub fn new(
            name: Span<String>,
            fields: Vec<PreparedField>,
            is_implicit: bool,
        ) -> Self {
            Self {
                name,
                is_copy: fields.iter().all(|f| f.ty.is_copy()),
                is_ref: fields.iter().any(|f| f.ty.is_ref()),
                is_named: !is_implicit || fields.len() > 1,
                fields,
            }
        }
    }
    pub(crate) struct PreparedType {
        pub(crate) name: String,
        pub(crate) struct_name: String,
        pub(crate) content: PreparedContent,
        pub(crate) is_copy: bool,
        pub(crate) is_params: bool,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PreparedType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PreparedType {
        #[inline]
        fn eq(&self, other: &PreparedType) -> bool {
            self.name == other.name && self.struct_name == other.struct_name
                && self.content == other.content && self.is_copy == other.is_copy
                && self.is_params == other.is_params
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PreparedType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PreparedType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<String>;
            let _: ::core::cmp::AssertParamIsEq<PreparedContent>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "PreparedType",
                "name",
                &&self.name,
                "struct_name",
                &&self.struct_name,
                "content",
                &&self.content,
                "is_copy",
                &&self.is_copy,
                "is_params",
                &&self.is_params,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedType {
        #[inline]
        fn clone(&self) -> PreparedType {
            PreparedType {
                name: ::core::clone::Clone::clone(&self.name),
                struct_name: ::core::clone::Clone::clone(&self.struct_name),
                content: ::core::clone::Clone::clone(&self.content),
                is_copy: ::core::clone::Clone::clone(&self.is_copy),
                is_params: ::core::clone::Clone::clone(&self.is_params),
            }
        }
    }
    pub(crate) enum PreparedContent {
        Enum(Vec<String>),
        Composite(Vec<PreparedField>),
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for PreparedContent {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for PreparedContent {
        #[inline]
        fn eq(&self, other: &PreparedContent) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        PreparedContent::Enum(__self_0),
                        PreparedContent::Enum(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    (
                        PreparedContent::Composite(__self_0),
                        PreparedContent::Composite(__arg1_0),
                    ) => *__self_0 == *__arg1_0,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for PreparedContent {}
    #[automatically_derived]
    impl ::core::cmp::Eq for PreparedContent {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Vec<String>>;
            let _: ::core::cmp::AssertParamIsEq<Vec<PreparedField>>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedContent {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                PreparedContent::Enum(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Enum",
                        &__self_0,
                    )
                }
                PreparedContent::Composite(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Composite",
                        &__self_0,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedContent {
        #[inline]
        fn clone(&self) -> PreparedContent {
            match self {
                PreparedContent::Enum(__self_0) => {
                    PreparedContent::Enum(::core::clone::Clone::clone(__self_0))
                }
                PreparedContent::Composite(__self_0) => {
                    PreparedContent::Composite(::core::clone::Clone::clone(__self_0))
                }
            }
        }
    }
    /// A struct containing the module name and the list of all
    /// the queries it contains.
    pub(crate) struct PreparedModule {
        pub(crate) info: ModuleInfo,
        pub(crate) queries: IndexMap<Span<String>, PreparedQuery>,
        pub(crate) params: IndexMap<Span<String>, PreparedItem>,
        pub(crate) rows: IndexMap<Span<String>, PreparedItem>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PreparedModule {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field4_finish(
                f,
                "PreparedModule",
                "info",
                &&self.info,
                "queries",
                &&self.queries,
                "params",
                &&self.params,
                "rows",
                &&self.rows,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PreparedModule {
        #[inline]
        fn clone(&self) -> PreparedModule {
            PreparedModule {
                info: ::core::clone::Clone::clone(&self.info),
                queries: ::core::clone::Clone::clone(&self.queries),
                params: ::core::clone::Clone::clone(&self.params),
                rows: ::core::clone::Clone::clone(&self.rows),
            }
        }
    }
    pub(crate) struct Preparation {
        pub(crate) modules: Vec<PreparedModule>,
        pub(crate) types: IndexMap<String, Vec<PreparedType>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Preparation {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "Preparation",
                "modules",
                &&self.modules,
                "types",
                &&self.types,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Preparation {
        #[inline]
        fn clone(&self) -> Preparation {
            Preparation {
                modules: ::core::clone::Clone::clone(&self.modules),
                types: ::core::clone::Clone::clone(&self.types),
            }
        }
    }
    impl PreparedModule {
        fn add(
            info: &ModuleInfo,
            map: &mut IndexMap<Span<String>, PreparedItem>,
            name: Span<String>,
            fields: Vec<PreparedField>,
            is_implicit: bool,
        ) -> Result<(usize, Vec<usize>), Error> {
            if !!fields.is_empty() {
                ::core::panicking::panic("assertion failed: !fields.is_empty()")
            }
            match map.entry(name.clone()) {
                Entry::Occupied(o) => {
                    let prev = &o.get();
                    let indexes: Vec<_> = if prev.is_named {
                        validation::named_struct_field(
                            info,
                            &prev.name,
                            &prev.fields,
                            &name,
                            &fields,
                        )?;
                        prev.fields
                            .iter()
                            .map(|f| fields.iter().position(|it| it == f).unwrap())
                            .collect()
                    } else {
                        <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([0]))
                    };
                    Ok((o.index(), indexes))
                }
                Entry::Vacant(v) => {
                    v.insert(
                        PreparedItem::new(name.clone(), fields.clone(), is_implicit),
                    );
                    Self::add(info, map, name, fields, is_implicit)
                }
            }
        }
        fn add_row(
            &mut self,
            name: Span<String>,
            fields: Vec<PreparedField>,
            is_implicit: bool,
        ) -> Result<(usize, Vec<usize>), Error> {
            let fuck = if fields.len() == 1 && is_implicit {
                name.map(|_| fields[0].unwrapped_name())
            } else {
                name
            };
            Self::add(&self.info, &mut self.rows, fuck, fields, is_implicit)
        }
        fn add_param(
            &mut self,
            name: Span<String>,
            fields: Vec<PreparedField>,
            is_implicit: bool,
        ) -> Result<(usize, Vec<usize>), Error> {
            Self::add(&self.info, &mut self.params, name, fields, is_implicit)
        }
        fn add_query(
            &mut self,
            name: Span<String>,
            param_idx: Option<(usize, Vec<usize>)>,
            row_idx: Option<(usize, Vec<usize>)>,
            sql: String,
        ) {
            self.queries
                .insert(
                    name.clone(),
                    PreparedQuery {
                        name: name.value,
                        row: row_idx,
                        sql,
                        param: param_idx,
                    },
                );
        }
    }
    /// Prepares all modules
    pub(crate) fn prepare(
        client: &mut Client,
        modules: Vec<Module>,
    ) -> Result<Preparation, Error> {
        let mut registrar = TypeRegistrar::default();
        let mut tmp = Preparation {
            modules: Vec::new(),
            types: IndexMap::new(),
        };
        let declared: Vec<_> = modules
            .iter()
            .flat_map(|it| &it.types)
            .map(|ty| (*ty).clone())
            .collect();
        for module in modules {
            tmp.modules.push(prepare_module(client, module, &mut registrar)?);
        }
        for ((schema, name), ty) in &registrar.types {
            if let Some(ty) = prepare_type(&registrar, name, ty, &declared) {
                match tmp.types.entry(schema.clone()) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(ty);
                    }
                    Entry::Vacant(entry) => {
                        entry
                            .insert(
                                <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([ty])),
                            );
                    }
                }
            }
        }
        Ok(tmp)
    }
    fn normalize_rust_name(name: &str) -> String {
        name.replace(':', "_")
    }
    /// Prepares database custom types
    fn prepare_type(
        registrar: &TypeRegistrar,
        name: &str,
        ty: &CornucopiaType,
        types: &[TypeAnnotation],
    ) -> Option<PreparedType> {
        if let CornucopiaType::Custom { pg_ty, struct_name, is_copy, is_params, .. }
            = ty {
            let declared = types
                .iter()
                .find(|it| it.name.value == pg_ty.name())
                .map_or(&[] as &[NullableIdent], |it| it.fields.as_slice());
            let content = match pg_ty.kind() {
                Kind::Enum(variants) => {
                    PreparedContent::Enum(
                        variants.clone().into_iter().map(escape_keyword).collect(),
                    )
                }
                Kind::Domain(_) => return None,
                Kind::Composite(fields) => {
                    PreparedContent::Composite(
                        fields
                            .iter()
                            .map(|field| {
                                let nullity = declared
                                    .iter()
                                    .find(|it| it.name.value == field.name());
                                PreparedField::new(
                                    field.name().to_string(),
                                    registrar.ref_of(field.type_()),
                                    nullity,
                                )
                            })
                            .collect(),
                    )
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            };
            Some(PreparedType {
                name: name.to_string(),
                struct_name: struct_name.clone(),
                content,
                is_copy: *is_copy,
                is_params: *is_params,
            })
        } else {
            None
        }
    }
    /// Prepares all queries in this module
    fn prepare_module(
        client: &mut Client,
        module: Module,
        registrar: &mut TypeRegistrar,
    ) -> Result<PreparedModule, Error> {
        validation::validate_module(&module)?;
        let mut tmp_prepared_module = PreparedModule {
            info: module.info.clone(),
            queries: IndexMap::new(),
            params: IndexMap::new(),
            rows: IndexMap::new(),
        };
        for query in module.queries {
            prepare_query(
                client,
                &mut tmp_prepared_module,
                registrar,
                &module.types,
                query,
                &module.info,
            )?;
        }
        validation::validate_preparation(&tmp_prepared_module)?;
        Ok(tmp_prepared_module)
    }
    /// Prepares a query
    fn prepare_query(
        client: &mut Client,
        module: &mut PreparedModule,
        registrar: &mut TypeRegistrar,
        types: &[TypeAnnotation],
        Query { name, param, bind_params, row, sql_str, sql_span }: Query,
        module_info: &ModuleInfo,
    ) -> Result<(), Error> {
        let stmt = client
            .prepare(&sql_str)
            .map_err(|e| Error::new_db_err(&e, module_info, &sql_span, &name))?;
        let (nullable_params_fields, params_name) = param
            .name_and_fields(types, &name, Some("Params"));
        let (nullable_row_fields, row_name) = row.name_and_fields(types, &name, None);
        let params_fields = {
            let stmt_params = stmt.params();
            let params = bind_params
                .iter()
                .zip(stmt_params)
                .map(|(a, b)| (a.clone(), b.clone()))
                .collect::<Vec<(Span<String>, Type)>>();
            validation::param_on_simple_query(
                &module.info,
                &name,
                &sql_span,
                &param,
                &params,
            )?;
            for nullable_col in nullable_params_fields {
                validation::nullable_param_name(&module.info, nullable_col, &params)
                    .map_err(Error::from)?;
            }
            let mut param_fields = Vec::new();
            for (col_name, col_ty) in params {
                let nullity = nullable_params_fields
                    .iter()
                    .find(|x| x.name.value == col_name.value);
                param_fields
                    .push(
                        PreparedField::new(
                            col_name.value.clone(),
                            registrar
                                .register(&col_name.value, &col_ty, &name, module_info)?
                                .clone(),
                            nullity,
                        ),
                    );
            }
            param_fields
        };
        let row_fields = {
            let stmt_cols = stmt.columns();
            validation::row_on_execute(&module.info, &name, &sql_span, &row, stmt_cols)?;
            validation::duplicate_sql_col_name(&module.info, &name, stmt_cols)
                .map_err(Error::from)?;
            for nullable_col in nullable_row_fields {
                validation::nullable_column_name(&module.info, nullable_col, stmt_cols)
                    .map_err(Error::from)?;
            }
            let mut row_fields = Vec::new();
            for (col_name, col_ty) in stmt_cols
                .iter()
                .map(|c| (c.name().to_owned(), c.type_()))
            {
                let nullity = nullable_row_fields
                    .iter()
                    .find(|x| x.name.value == col_name);
                let ty = registrar
                    .register(&col_name, col_ty, &name, module_info)?
                    .clone();
                row_fields
                    .push(
                        PreparedField::new(normalize_rust_name(&col_name), ty, nullity),
                    );
            }
            row_fields
        };
        let row_idx = if row_fields.is_empty() {
            None
        } else {
            Some(module.add_row(row_name, row_fields, row.is_implicit())?)
        };
        let param_idx = if params_fields.is_empty() {
            None
        } else {
            Some(module.add_param(params_name, params_fields, param.is_implicit())?)
        };
        module.add_query(name.clone(), param_idx, row_idx, sql_str);
        Ok(())
    }
    pub(crate) mod error {
        use miette::{Diagnostic, NamedSource, SourceSpan};
        use thiserror::Error as ThisError;
        use crate::{
            parser::Span, read_queries::ModuleInfo,
            type_registrar::error::Error as PostgresTypeError, utils::db_err,
            validation::error::Error as ValidationError,
        };
        pub enum Error {
            #[error("Couldn't prepare query: {msg}")]
            Db {
                msg: String,
                #[help]
                help: Option<String>,
                #[source_code]
                src: NamedSource,
                #[label("error occurs near this location")]
                err_span: Option<SourceSpan>,
            },
            #[error(transparent)]
            #[diagnostic(transparent)]
            PostgresType(#[from] PostgresTypeError),
            #[error(transparent)]
            #[diagnostic(transparent)]
            Validation(#[from] ValidationError),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::Db {
                        msg: __self_0,
                        help: __self_1,
                        src: __self_2,
                        err_span: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "Db",
                            "msg",
                            &__self_0,
                            "help",
                            &__self_1,
                            "src",
                            &__self_2,
                            "err_span",
                            &__self_3,
                        )
                    }
                    Error::PostgresType(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "PostgresType",
                            &__self_0,
                        )
                    }
                    Error::Validation(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Validation",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::__private::AsDynError;
                #[allow(deprecated)]
                match self {
                    Error::Db { .. } => std::option::Option::None,
                    Error::PostgresType { 0: transparent } => {
                        std::error::Error::source(transparent.as_dyn_error())
                    }
                    Error::Validation { 0: transparent } => {
                        std::error::Error::source(transparent.as_dyn_error())
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::Db { msg, help, src, err_span } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t prepare query: "],
                                    &[::core::fmt::ArgumentV1::new_display(&msg.as_display())],
                                ),
                            )
                    }
                    Error::PostgresType(_0) => std::fmt::Display::fmt(_0, __formatter),
                    Error::Validation(_0) => std::fmt::Display::fmt(_0, __formatter),
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::convert::From<PostgresTypeError> for Error {
            #[allow(deprecated)]
            fn from(source: PostgresTypeError) -> Self {
                Error::PostgresType { 0: source }
            }
        }
        #[allow(unused_qualifications)]
        impl std::convert::From<ValidationError> for Error {
            #[allow(deprecated)]
            fn from(source: ValidationError) -> Self {
                Error::Validation { 0: source }
            }
        }
        impl miette::Diagnostic for Error {
            fn code<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::PostgresType(unnamed, ..) => unnamed.code(),
                    Self::Validation(unnamed, ..) => unnamed.code(),
                    _ => std::option::Option::None,
                }
            }
            fn help<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Db { msg, help, src, err_span } => {
                        use miette::macro_helpers::ToOption;
                        miette::macro_helpers::OptionalWrapper::<Option<String>>::new()
                            .to_option(&help)
                            .as_ref()
                            .map(|
                                __miette_internal_var,
                            | -> std::boxed::Box<dyn std::fmt::Display + 'a> {
                                std::boxed::Box::new({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &[""],
                                            &[
                                                ::core::fmt::ArgumentV1::new_display(&__miette_internal_var),
                                            ],
                                        ),
                                    );
                                    res
                                })
                            })
                    }
                    Self::PostgresType(unnamed, ..) => unnamed.help(),
                    Self::Validation(unnamed, ..) => unnamed.help(),
                    _ => std::option::Option::None,
                }
            }
            fn severity(&self) -> std::option::Option<miette::Severity> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::PostgresType(unnamed, ..) => unnamed.severity(),
                    Self::Validation(unnamed, ..) => unnamed.severity(),
                    _ => std::option::Option::None,
                }
            }
            fn labels(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
            > {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Db { msg, help, src, err_span } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<
                                                Option<SourceSpan>,
                                            >::new()
                                                .to_option(err_span)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["error occurs near this location"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::PostgresType(unnamed, ..) => unnamed.labels(),
                    Self::Validation(unnamed, ..) => unnamed.labels(),
                    _ => std::option::Option::None,
                }
            }
            fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::Db { msg, help, src, err_span } => {
                        std::option::Option::Some(src)
                    }
                    Self::PostgresType(unnamed, ..) => unnamed.source_code(),
                    Self::Validation(unnamed, ..) => unnamed.source_code(),
                    _ => std::option::Option::None,
                }
            }
            fn related(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<
                    dyn std::iter::Iterator<Item = &dyn miette::Diagnostic> + '_,
                >,
            > {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::PostgresType(unnamed, ..) => unnamed.related(),
                    Self::Validation(unnamed, ..) => unnamed.related(),
                    _ => std::option::Option::None,
                }
            }
            fn url<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::PostgresType(unnamed, ..) => unnamed.url(),
                    Self::Validation(unnamed, ..) => unnamed.url(),
                    _ => std::option::Option::None,
                }
            }
            fn diagnostic_source(&self) -> std::option::Option<&dyn miette::Diagnostic> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::PostgresType(unnamed, ..) => unnamed.diagnostic_source(),
                    Self::Validation(unnamed, ..) => unnamed.diagnostic_source(),
                    _ => std::option::Option::None,
                }
            }
        }
        impl Error {
            pub(crate) fn new_db_err(
                err: &postgres::Error,
                module_info: &ModuleInfo,
                query_span: &SourceSpan,
                query_name: &Span<String>,
            ) -> Self {
                let msg = {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&err)],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    );
                    res
                };
                if let Some((position, msg, help)) = db_err(err) {
                    Self::Db {
                        msg,
                        help,
                        src: module_info.into(),
                        err_span: Some(
                            (query_span.offset() + position as usize - 1).into(),
                        ),
                    }
                } else {
                    Self::Db {
                        msg,
                        help: None,
                        src: module_info.into(),
                        err_span: Some(query_name.span),
                    }
                }
            }
        }
    }
}
mod read_queries {
    use miette::NamedSource;
    use self::error::Error;
    pub(crate) struct ModuleInfo {
        pub(crate) path: String,
        pub(crate) name: String,
        pub(crate) content: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ModuleInfo {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ModuleInfo",
                "path",
                &&self.path,
                "name",
                &&self.name,
                "content",
                &&self.content,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ModuleInfo {
        #[inline]
        fn clone(&self) -> ModuleInfo {
            ModuleInfo {
                path: ::core::clone::Clone::clone(&self.path),
                name: ::core::clone::Clone::clone(&self.name),
                content: ::core::clone::Clone::clone(&self.content),
            }
        }
    }
    impl From<ModuleInfo> for NamedSource {
        fn from(m: ModuleInfo) -> Self {
            Self::new(m.path, m.content)
        }
    }
    impl From<&ModuleInfo> for NamedSource {
        fn from(m: &ModuleInfo) -> Self {
            Self::new(&m.path, m.content.clone())
        }
    }
    /// Reads queries in the directory. Only .sql files are considered.
    ///
    /// # Error
    /// Returns an error if `dir_path` does not point to a valid directory or if a query file cannot be parsed.
    pub(crate) fn read_query_modules(dir_path: &str) -> Result<Vec<ModuleInfo>, Error> {
        let mut modules_info = Vec::new();
        for entry_result in std::fs::read_dir(dir_path)
            .map_err(|err| Error {
                err,
                path: String::from(dir_path),
            })?
        {
            let entry = entry_result
                .map_err(|err| Error {
                    err,
                    path: dir_path.to_owned(),
                })?;
            let path_buf = entry.path();
            if path_buf
                .extension()
                .map(|extension| extension == "sql")
                .unwrap_or_default()
            {
                let module_name = path_buf
                    .file_stem()
                    .expect("is a file")
                    .to_str()
                    .expect("file name is valid utf8")
                    .to_string();
                let file_contents = std::fs::read_to_string(&path_buf)
                    .map_err(|err| Error {
                        err,
                        path: dir_path.to_owned(),
                    })?;
                modules_info
                    .push(ModuleInfo {
                        path: String::from(path_buf.to_string_lossy()),
                        name: module_name,
                        content: file_contents,
                    });
            }
        }
        modules_info.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(modules_info)
    }
    pub(crate) mod error {
        use miette::Diagnostic;
        use thiserror::Error as ThisError;
        #[error("[{path}] : {err:#}")]
        pub struct Error {
            pub(crate) err: std::io::Error,
            pub(crate) path: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Error",
                    "err",
                    &&self.err,
                    "path",
                    &&self.path,
                )
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { err, path } = self;
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &["[", "] : "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&path.as_display()),
                                ::core::fmt::ArgumentV1::new_display(&err),
                            ],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    )
            }
        }
        impl miette::Diagnostic for Error {}
    }
}
mod type_registrar {
    use std::rc::Rc;
    use heck::ToUpperCamelCase;
    use indexmap::{map::Entry, IndexMap};
    use postgres_types::{Kind, Type};
    use crate::{
        codegen::idx_char, parser::Span, read_queries::ModuleInfo, utils::SchemaKey,
    };
    use self::error::Error;
    /// A struct containing a postgres type and its Rust-equivalent.
    pub(crate) enum CornucopiaType {
        Simple { pg_ty: Type, rust_name: &'static str, is_copy: bool },
        Array { inner: Rc<CornucopiaType> },
        Domain { pg_ty: Type, inner: Rc<CornucopiaType> },
        Custom {
            pg_ty: Type,
            struct_name: String,
            struct_path: String,
            is_copy: bool,
            is_params: bool,
        },
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for CornucopiaType {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for CornucopiaType {
        #[inline]
        fn eq(&self, other: &CornucopiaType) -> bool {
            let __self_tag = ::core::intrinsics::discriminant_value(self);
            let __arg1_tag = ::core::intrinsics::discriminant_value(other);
            __self_tag == __arg1_tag
                && match (self, other) {
                    (
                        CornucopiaType::Simple {
                            pg_ty: __self_0,
                            rust_name: __self_1,
                            is_copy: __self_2,
                        },
                        CornucopiaType::Simple {
                            pg_ty: __arg1_0,
                            rust_name: __arg1_1,
                            is_copy: __arg1_2,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2
                    }
                    (
                        CornucopiaType::Array { inner: __self_0 },
                        CornucopiaType::Array { inner: __arg1_0 },
                    ) => *__self_0 == *__arg1_0,
                    (
                        CornucopiaType::Domain { pg_ty: __self_0, inner: __self_1 },
                        CornucopiaType::Domain { pg_ty: __arg1_0, inner: __arg1_1 },
                    ) => *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1,
                    (
                        CornucopiaType::Custom {
                            pg_ty: __self_0,
                            struct_name: __self_1,
                            struct_path: __self_2,
                            is_copy: __self_3,
                            is_params: __self_4,
                        },
                        CornucopiaType::Custom {
                            pg_ty: __arg1_0,
                            struct_name: __arg1_1,
                            struct_path: __arg1_2,
                            is_copy: __arg1_3,
                            is_params: __arg1_4,
                        },
                    ) => {
                        *__self_0 == *__arg1_0 && *__self_1 == *__arg1_1
                            && *__self_2 == *__arg1_2 && *__self_3 == *__arg1_3
                            && *__self_4 == *__arg1_4
                    }
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralEq for CornucopiaType {}
    #[automatically_derived]
    impl ::core::cmp::Eq for CornucopiaType {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<Type>;
            let _: ::core::cmp::AssertParamIsEq<&'static str>;
            let _: ::core::cmp::AssertParamIsEq<bool>;
            let _: ::core::cmp::AssertParamIsEq<Rc<CornucopiaType>>;
            let _: ::core::cmp::AssertParamIsEq<Rc<CornucopiaType>>;
            let _: ::core::cmp::AssertParamIsEq<String>;
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CornucopiaType {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                CornucopiaType::Simple {
                    pg_ty: __self_0,
                    rust_name: __self_1,
                    is_copy: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Simple",
                        "pg_ty",
                        &__self_0,
                        "rust_name",
                        &__self_1,
                        "is_copy",
                        &__self_2,
                    )
                }
                CornucopiaType::Array { inner: __self_0 } => {
                    ::core::fmt::Formatter::debug_struct_field1_finish(
                        f,
                        "Array",
                        "inner",
                        &__self_0,
                    )
                }
                CornucopiaType::Domain { pg_ty: __self_0, inner: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Domain",
                        "pg_ty",
                        &__self_0,
                        "inner",
                        &__self_1,
                    )
                }
                CornucopiaType::Custom {
                    pg_ty: __self_0,
                    struct_name: __self_1,
                    struct_path: __self_2,
                    is_copy: __self_3,
                    is_params: __self_4,
                } => {
                    ::core::fmt::Formatter::debug_struct_field5_finish(
                        f,
                        "Custom",
                        "pg_ty",
                        &__self_0,
                        "struct_name",
                        &__self_1,
                        "struct_path",
                        &__self_2,
                        "is_copy",
                        &__self_3,
                        "is_params",
                        &__self_4,
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CornucopiaType {
        #[inline]
        fn clone(&self) -> CornucopiaType {
            match self {
                CornucopiaType::Simple {
                    pg_ty: __self_0,
                    rust_name: __self_1,
                    is_copy: __self_2,
                } => {
                    CornucopiaType::Simple {
                        pg_ty: ::core::clone::Clone::clone(__self_0),
                        rust_name: ::core::clone::Clone::clone(__self_1),
                        is_copy: ::core::clone::Clone::clone(__self_2),
                    }
                }
                CornucopiaType::Array { inner: __self_0 } => {
                    CornucopiaType::Array {
                        inner: ::core::clone::Clone::clone(__self_0),
                    }
                }
                CornucopiaType::Domain { pg_ty: __self_0, inner: __self_1 } => {
                    CornucopiaType::Domain {
                        pg_ty: ::core::clone::Clone::clone(__self_0),
                        inner: ::core::clone::Clone::clone(__self_1),
                    }
                }
                CornucopiaType::Custom {
                    pg_ty: __self_0,
                    struct_name: __self_1,
                    struct_path: __self_2,
                    is_copy: __self_3,
                    is_params: __self_4,
                } => {
                    CornucopiaType::Custom {
                        pg_ty: ::core::clone::Clone::clone(__self_0),
                        struct_name: ::core::clone::Clone::clone(__self_1),
                        struct_path: ::core::clone::Clone::clone(__self_2),
                        is_copy: ::core::clone::Clone::clone(__self_3),
                        is_params: ::core::clone::Clone::clone(__self_4),
                    }
                }
            }
        }
    }
    impl CornucopiaType {
        /// Is this type need a generic lifetime
        pub fn is_ref(&self) -> bool {
            match self {
                CornucopiaType::Simple { pg_ty, .. } => {
                    match *pg_ty {
                        Type::BYTEA
                        | Type::TEXT
                        | Type::VARCHAR
                        | Type::JSON
                        | Type::JSONB => false,
                        _ => !self.is_copy(),
                    }
                }
                CornucopiaType::Domain { inner, .. }
                | CornucopiaType::Array { inner } => inner.is_ref(),
                _ => !self.is_copy(),
            }
        }
        /// Is this type copyable
        pub fn is_copy(&self) -> bool {
            match self {
                CornucopiaType::Simple { is_copy, .. }
                | CornucopiaType::Custom { is_copy, .. } => *is_copy,
                CornucopiaType::Domain { inner, .. } => inner.is_copy(),
                CornucopiaType::Array { .. } => false,
            }
        }
        /// Can this used in parameters as it is
        pub fn is_params(&self) -> bool {
            match self {
                CornucopiaType::Simple { .. } => true,
                CornucopiaType::Array { .. } => false,
                CornucopiaType::Domain { inner, .. } => inner.is_params(),
                CornucopiaType::Custom { is_params, .. } => *is_params,
            }
        }
        /// Wrap type to escape domains in parameters
        pub(crate) fn sql_wrapped(&self, name: &str, is_async: bool) -> String {
            let client_name = if is_async { "async" } else { "sync" };
            match self {
                CornucopiaType::Domain { inner, .. } => {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["&cornucopia_", "::private::Domain(", ")"],
                            &match (&inner.sql_wrapped(name, is_async), &client_name) {
                                args => {
                                    [
                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                    ]
                                }
                            },
                        ),
                    );
                    res
                }
                CornucopiaType::Array { inner } => {
                    match inner.as_ref() {
                        CornucopiaType::Domain { inner, .. } => {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["&cornucopia_", "::private::DomainArray(", ")"],
                                    &match (&inner.sql_wrapped(name, is_async), &client_name) {
                                        args => {
                                            [
                                                ::core::fmt::ArgumentV1::new_display(args.1),
                                                ::core::fmt::ArgumentV1::new_display(args.0),
                                            ]
                                        }
                                    },
                                ),
                            );
                            res
                        }
                        _ => name.to_string(),
                    }
                }
                _ => name.to_string(),
            }
        }
        /// Wrap type to escape domains when writing to sql
        pub(crate) fn accept_to_sql(&self, is_async: bool) -> String {
            let client_name = if is_async { "async" } else { "sync" };
            match self {
                CornucopiaType::Domain { inner, .. } => {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["cornucopia_", "::private::Domain::<", ">"],
                            &match (&inner.accept_to_sql(is_async), &client_name) {
                                args => {
                                    [
                                        ::core::fmt::ArgumentV1::new_display(args.1),
                                        ::core::fmt::ArgumentV1::new_display(args.0),
                                    ]
                                }
                            },
                        ),
                    );
                    res
                }
                CornucopiaType::Array { inner } => {
                    match inner.as_ref() {
                        CornucopiaType::Domain { inner, .. } => {
                            let ty = inner.accept_to_sql(is_async);
                            {
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["cornucopia_", "::private::DomainArray::<", ", &[", "]>"],
                                        &match (&ty, &client_name, &ty) {
                                            args => {
                                                [
                                                    ::core::fmt::ArgumentV1::new_display(args.1),
                                                    ::core::fmt::ArgumentV1::new_display(args.0),
                                                    ::core::fmt::ArgumentV1::new_display(args.2),
                                                ]
                                            }
                                        },
                                    ),
                                );
                                res
                            }
                        }
                        _ => self.param_ty(false, is_async),
                    }
                }
                _ => self.param_ty(false, is_async),
            }
        }
        /// Corresponding postgres type
        pub(crate) fn pg_ty(&self) -> &Type {
            match self {
                CornucopiaType::Simple { pg_ty, .. }
                | CornucopiaType::Custom { pg_ty, .. }
                | CornucopiaType::Domain { pg_ty, .. } => pg_ty,
                CornucopiaType::Array { inner } => inner.pg_ty(),
            }
        }
        /// Code to transform its borrowed type to its owned one
        pub(crate) fn owning_call(
            &self,
            name: &str,
            is_nullable: bool,
            is_inner_nullable: bool,
        ) -> String {
            if self.is_copy() {
                return name.into();
            }
            if is_nullable {
                let into = self.owning_call("v", false, is_inner_nullable);
                return {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", ".map(|v| ", ")"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&name),
                                ::core::fmt::ArgumentV1::new_display(&into),
                            ],
                        ),
                    );
                    res
                };
            }
            match self {
                CornucopiaType::Simple {
                    pg_ty,
                    ..
                } if match *pg_ty {
                    Type::JSON | Type::JSONB => true,
                    _ => false,
                } => {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["serde_json::from_str(", ".0.get()).unwrap()"],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    );
                    res
                }
                CornucopiaType::Array { inner, .. } => {
                    let inner = inner.owning_call("v", is_inner_nullable, false);
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["", ".map(|v| ", ").collect()"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&name),
                                    ::core::fmt::ArgumentV1::new_display(&inner),
                                ],
                            ),
                        );
                        res
                    }
                }
                CornucopiaType::Domain { inner, .. } => {
                    inner.owning_call(name, is_nullable, false)
                }
                _ => {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", ".into()"],
                            &[::core::fmt::ArgumentV1::new_display(&name)],
                        ),
                    );
                    res
                }
            }
        }
        /// Corresponding owned type
        pub(crate) fn own_ty(&self, is_inner_nullable: bool) -> String {
            match self {
                CornucopiaType::Simple { rust_name, .. } => (*rust_name).to_string(),
                CornucopiaType::Array { inner, .. } => {
                    let own_inner = inner.own_ty(false);
                    if is_inner_nullable {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Vec<Option<", ">>"],
                                    &[::core::fmt::ArgumentV1::new_display(&own_inner)],
                                ),
                            );
                            res
                        }
                    } else {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Vec<", ">"],
                                    &[::core::fmt::ArgumentV1::new_display(&own_inner)],
                                ),
                            );
                            res
                        }
                    }
                }
                CornucopiaType::Domain { inner, .. } => inner.own_ty(false),
                CornucopiaType::Custom { struct_path, .. } => struct_path.to_string(),
            }
        }
        /// Corresponding borrowed ergonomic parameter type (using traits if possible)
        pub(crate) fn param_ergo_ty(
            &self,
            is_inner_nullable: bool,
            is_async: bool,
            traits: &mut Vec<String>,
        ) -> String {
            let client_name = if is_async { "async" } else { "sync" };
            match self {
                CornucopiaType::Simple { pg_ty, .. } => {
                    match *pg_ty {
                        Type::BYTEA => {
                            traits
                                .push({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["cornucopia_", "::BytesSql"],
                                            &[::core::fmt::ArgumentV1::new_display(&client_name)],
                                        ),
                                    );
                                    res
                                });
                            idx_char(traits.len())
                        }
                        Type::TEXT | Type::VARCHAR => {
                            traits
                                .push({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["cornucopia_", "::StringSql"],
                                            &[::core::fmt::ArgumentV1::new_display(&client_name)],
                                        ),
                                    );
                                    res
                                });
                            idx_char(traits.len())
                        }
                        Type::JSON | Type::JSONB => {
                            traits
                                .push({
                                    let res = ::alloc::fmt::format(
                                        ::core::fmt::Arguments::new_v1(
                                            &["cornucopia_", "::JsonSql"],
                                            &[::core::fmt::ArgumentV1::new_display(&client_name)],
                                        ),
                                    );
                                    res
                                });
                            idx_char(traits.len())
                        }
                        _ => self.param_ty(is_inner_nullable, is_async),
                    }
                }
                CornucopiaType::Array { inner, .. } => {
                    let inner = inner.param_ergo_ty(is_inner_nullable, is_async, traits);
                    let inner = if is_inner_nullable {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Option<", ">"],
                                    &[::core::fmt::ArgumentV1::new_display(&inner)],
                                ),
                            );
                            res
                        }
                    } else {
                        inner
                    };
                    traits
                        .push({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["cornucopia_", "::ArraySql<Item = ", ">"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&client_name),
                                        ::core::fmt::ArgumentV1::new_display(&inner),
                                    ],
                                ),
                            );
                            res
                        });
                    idx_char(traits.len())
                }
                CornucopiaType::Domain { inner, .. } => {
                    inner.param_ergo_ty(is_inner_nullable, is_async, traits)
                }
                CornucopiaType::Custom { .. } => {
                    self.param_ty(is_inner_nullable, is_async)
                }
            }
        }
        /// Corresponding borrowed parameter type
        pub(crate) fn param_ty(
            &self,
            is_inner_nullable: bool,
            is_async: bool,
        ) -> String {
            match self {
                CornucopiaType::Simple { pg_ty, .. } => {
                    match *pg_ty {
                        Type::JSON | Type::JSONB => {
                            "&'a serde_json::value::Value".to_string()
                        }
                        _ => self.brw_ty(is_inner_nullable, true, is_async),
                    }
                }
                CornucopiaType::Array { inner, .. } => {
                    let inner = inner.param_ty(is_inner_nullable, is_async);
                    let inner = if is_inner_nullable {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Option<", ">"],
                                    &[::core::fmt::ArgumentV1::new_display(&inner)],
                                ),
                            );
                            res
                        }
                    } else {
                        inner
                    };
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["&\'a [", "]"],
                                &[::core::fmt::ArgumentV1::new_display(&inner)],
                            ),
                        );
                        res
                    }
                }
                CornucopiaType::Domain { inner, .. } => inner.param_ty(false, is_async),
                CornucopiaType::Custom { struct_path, is_params, is_copy, .. } => {
                    if !is_copy && !is_params {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "Params<\'a>"],
                                    &[::core::fmt::ArgumentV1::new_display(&struct_path)],
                                ),
                            );
                            res
                        }
                    } else {
                        self.brw_ty(is_inner_nullable, true, is_async)
                    }
                }
            }
        }
        /// String representing a borrowed rust equivalent of this type. Notably, if
        /// a Rust equivalent is a String or a Vec<T>, it will return a &str and a &[T] respectively.
        pub(crate) fn brw_ty(
            &self,
            is_inner_nullable: bool,
            has_lifetime: bool,
            is_async: bool,
        ) -> String {
            let client_name = if is_async { "async" } else { "sync" };
            let lifetime = if has_lifetime { "'a" } else { "" };
            match self {
                CornucopiaType::Simple { pg_ty, rust_name, .. } => {
                    match *pg_ty {
                        Type::BYTEA => {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["&", " [u8]"],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            );
                            res
                        }
                        Type::TEXT | Type::VARCHAR => {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["&", " str"],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            );
                            res
                        }
                        Type::JSON | Type::JSONB => {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "postgres_types::Json<&",
                                        " serde_json::value::RawValue>",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&lifetime)],
                                ),
                            );
                            res
                        }
                        _ => (*rust_name).to_string(),
                    }
                }
                CornucopiaType::Array { inner, .. } => {
                    let inner = inner.brw_ty(is_inner_nullable, has_lifetime, is_async);
                    let inner = if is_inner_nullable {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["Option<", ">"],
                                    &[::core::fmt::ArgumentV1::new_display(&inner)],
                                ),
                            );
                            res
                        }
                    } else {
                        inner
                    };
                    let lifetime = if has_lifetime { lifetime } else { "'_" };
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["cornucopia_", "::ArrayIterator<", ", ", ">"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&client_name),
                                    ::core::fmt::ArgumentV1::new_display(&lifetime),
                                    ::core::fmt::ArgumentV1::new_display(&inner),
                                ],
                            ),
                        );
                        res
                    }
                }
                CornucopiaType::Domain { inner, .. } => {
                    inner.brw_ty(false, has_lifetime, is_async)
                }
                CornucopiaType::Custom { struct_path, is_copy, .. } => {
                    if *is_copy {
                        struct_path.to_string()
                    } else {
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "Borrowed<", ">"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&struct_path),
                                        ::core::fmt::ArgumentV1::new_display(&lifetime),
                                    ],
                                ),
                            );
                            res
                        }
                    }
                }
            }
        }
    }
    /// Data structure holding all types known to this particular run of Cornucopia.
    pub(crate) struct TypeRegistrar {
        pub types: IndexMap<(String, String), Rc<CornucopiaType>>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TypeRegistrar {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "TypeRegistrar",
                "types",
                &&self.types,
            )
        }
    }
    #[automatically_derived]
    impl ::core::clone::Clone for TypeRegistrar {
        #[inline]
        fn clone(&self) -> TypeRegistrar {
            TypeRegistrar {
                types: ::core::clone::Clone::clone(&self.types),
            }
        }
    }
    #[automatically_derived]
    impl ::core::default::Default for TypeRegistrar {
        #[inline]
        fn default() -> TypeRegistrar {
            TypeRegistrar {
                types: ::core::default::Default::default(),
            }
        }
    }
    impl TypeRegistrar {
        pub(crate) fn register(
            &mut self,
            name: &str,
            ty: &Type,
            query_name: &Span<String>,
            module_info: &ModuleInfo,
        ) -> Result<&Rc<CornucopiaType>, Error> {
            fn custom(ty: &Type, is_copy: bool, is_params: bool) -> CornucopiaType {
                let rust_ty_name = ty.name().to_upper_camel_case();
                CornucopiaType::Custom {
                    pg_ty: ty.clone(),
                    struct_path: {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["super::super::types::", "::"],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&ty.schema()),
                                    ::core::fmt::ArgumentV1::new_display(&rust_ty_name),
                                ],
                            ),
                        );
                        res
                    },
                    struct_name: rust_ty_name,
                    is_copy,
                    is_params,
                }
            }
            fn domain(ty: &Type, inner: Rc<CornucopiaType>) -> CornucopiaType {
                CornucopiaType::Domain {
                    pg_ty: ty.clone(),
                    inner,
                }
            }
            if let Some(idx) = self.types.get_index_of(&SchemaKey::from(ty)) {
                return Ok(&self.types[idx]);
            }
            Ok(
                match ty.kind() {
                    Kind::Enum(_) => self.insert(ty, || custom(ty, true, true)),
                    Kind::Array(inner_ty) => {
                        let inner = self
                            .register(name, inner_ty, query_name, module_info)?
                            .clone();
                        self.insert(
                            ty,
                            || CornucopiaType::Array {
                                inner: inner.clone(),
                            },
                        )
                    }
                    Kind::Domain(inner_ty) => {
                        let inner = self
                            .register(name, inner_ty, query_name, module_info)?
                            .clone();
                        self.insert(ty, || domain(ty, inner.clone()))
                    }
                    Kind::Composite(composite_fields) => {
                        let mut is_copy = true;
                        let mut is_params = true;
                        for field in composite_fields {
                            let field_ty = self
                                .register(name, field.type_(), query_name, module_info)?;
                            is_copy &= field_ty.is_copy();
                            is_params &= field_ty.is_params();
                        }
                        self.insert(ty, || custom(ty, is_copy, is_params))
                    }
                    Kind::Simple => {
                        let (rust_name, is_copy) = match *ty {
                            Type::BOOL => ("bool", true),
                            Type::CHAR => ("i8", true),
                            Type::INT2 => ("i16", true),
                            Type::INT4 => ("i32", true),
                            Type::INT8 => ("i64", true),
                            Type::FLOAT4 => ("f32", true),
                            Type::FLOAT8 => ("f64", true),
                            Type::TEXT | Type::VARCHAR => ("String", false),
                            Type::BYTEA => ("Vec<u8>", false),
                            Type::TIMESTAMP => ("time::PrimitiveDateTime", true),
                            Type::TIMESTAMPTZ => ("time::OffsetDateTime", true),
                            Type::DATE => ("time::Date", true),
                            Type::TIME => ("time::Time", true),
                            Type::JSON | Type::JSONB => ("serde_json::Value", false),
                            Type::UUID => ("uuid::Uuid", true),
                            Type::INET => ("std::net::IpAddr", true),
                            Type::MACADDR => ("eui48::MacAddress", true),
                            Type::NUMERIC => ("rust_decimal::Decimal", true),
                            _ => {
                                return Err(Error::UnsupportedPostgresType {
                                    src: module_info.clone().into(),
                                    query: query_name.span,
                                    col_name: name.to_string(),
                                    col_ty: ty.to_string(),
                                });
                            }
                        };
                        self.insert(
                            ty,
                            || CornucopiaType::Simple {
                                pg_ty: ty.clone(),
                                rust_name,
                                is_copy,
                            },
                        )
                    }
                    _ => {
                        return Err(Error::UnsupportedPostgresType {
                            src: module_info.clone().into(),
                            query: query_name.span,
                            col_name: name.to_string(),
                            col_ty: ty.to_string(),
                        });
                    }
                },
            )
        }
        pub(crate) fn ref_of(&self, ty: &Type) -> Rc<CornucopiaType> {
            self.types
                .get(&SchemaKey::from(ty))
                .expect("type must already be registered")
                .clone()
        }
        fn insert(
            &mut self,
            ty: &Type,
            call: impl Fn() -> CornucopiaType,
        ) -> &Rc<CornucopiaType> {
            let index = match self
                .types
                .entry((ty.schema().to_owned(), ty.name().to_owned()))
            {
                Entry::Occupied(o) => o.index(),
                Entry::Vacant(v) => {
                    let index = v.index();
                    v.insert(Rc::new(call()));
                    index
                }
            };
            &self.types[index]
        }
    }
    impl std::ops::Index<&Type> for TypeRegistrar {
        type Output = Rc<CornucopiaType>;
        fn index(&self, index: &Type) -> &Self::Output {
            &self.types[&SchemaKey::from(index)]
        }
    }
    pub(crate) mod error {
        use miette::{Diagnostic, NamedSource, SourceSpan};
        use thiserror::Error as ThisError;
        #[error("Couldn't register SQL type.")]
        pub enum Error {
            Db(#[from] postgres::Error),
            UnsupportedPostgresType {
                #[source_code]
                src: NamedSource,
                #[label(
                    "this query contains an unsupported type (name: {col_name}, type: {col_ty})"
                )]
                query: SourceSpan,
                col_name: String,
                col_ty: String,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::Db(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Db",
                            &__self_0,
                        )
                    }
                    Error::UnsupportedPostgresType {
                        src: __self_0,
                        query: __self_1,
                        col_name: __self_2,
                        col_ty: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "UnsupportedPostgresType",
                            "src",
                            &__self_0,
                            "query",
                            &__self_1,
                            "col_name",
                            &__self_2,
                            "col_ty",
                            &__self_3,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::__private::AsDynError;
                #[allow(deprecated)]
                match self {
                    Error::Db { 0: source, .. } => {
                        std::option::Option::Some(source.as_dyn_error())
                    }
                    Error::UnsupportedPostgresType { .. } => std::option::Option::None,
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::Db(_0) => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t register SQL type."],
                                    &[],
                                ),
                            )
                    }
                    Error::UnsupportedPostgresType { src, query, col_name, col_ty } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["Couldn\'t register SQL type."],
                                    &[],
                                ),
                            )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::convert::From<postgres::Error> for Error {
            #[allow(deprecated)]
            fn from(source: postgres::Error) -> Self {
                Error::Db { 0: source }
            }
        }
        impl miette::Diagnostic for Error {
            fn labels(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
            > {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::UnsupportedPostgresType { src, query, col_name, col_ty } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(query)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &[
                                                                    "this query contains an unsupported type (name: ",
                                                                    ", type: ",
                                                                    ")",
                                                                ],
                                                                &[
                                                                    ::core::fmt::ArgumentV1::new_display(&col_name),
                                                                    ::core::fmt::ArgumentV1::new_display(&col_ty),
                                                                ],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    _ => std::option::Option::None,
                }
            }
            fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::UnsupportedPostgresType { src, query, col_name, col_ty } => {
                        std::option::Option::Some(src)
                    }
                    _ => std::option::Option::None,
                }
            }
        }
    }
}
mod utils {
    use std::{cell::RefCell, fmt::{Display, Formatter}};
    use indexmap::Equivalent;
    use postgres::error::ErrorPosition;
    use postgres_types::Type;
    pub struct Lazy<F: Fn(&mut Formatter)> {
        f: RefCell<Option<F>>,
    }
    impl<F: Fn(&mut Formatter)> Lazy<F> {
        pub fn new(f: F) -> Self {
            Self { f: RefCell::new(Some(f)) }
        }
    }
    impl<F: Fn(&mut Formatter)> Display for Lazy<F> {
        fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
            if let Some(f) = self.f.take() {
                f(fmt);
            }
            Ok(())
        }
    }
    /// Allows us to query a map using type schema as key without having to own the key strings
    pub struct SchemaKey<'a> {
        schema: &'a str,
        name: &'a str,
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralPartialEq for SchemaKey<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::PartialEq for SchemaKey<'a> {
        #[inline]
        fn eq(&self, other: &SchemaKey<'a>) -> bool {
            self.schema == other.schema && self.name == other.name
        }
    }
    #[automatically_derived]
    impl<'a> ::core::marker::StructuralEq for SchemaKey<'a> {}
    #[automatically_derived]
    impl<'a> ::core::cmp::Eq for SchemaKey<'a> {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
            let _: ::core::cmp::AssertParamIsEq<&'a str>;
        }
    }
    #[automatically_derived]
    impl<'a> ::core::hash::Hash for SchemaKey<'a> {
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            ::core::hash::Hash::hash(&self.schema, state);
            ::core::hash::Hash::hash(&self.name, state)
        }
    }
    impl<'a> From<&'a Type> for SchemaKey<'a> {
        fn from(ty: &'a Type) -> Self {
            SchemaKey {
                schema: ty.schema(),
                name: ty.name(),
            }
        }
    }
    impl<'a> Equivalent<(String, String)> for SchemaKey<'a> {
        fn equivalent(&self, key: &(String, String)) -> bool {
            key.0.as_str().equivalent(&self.schema)
                && key.1.as_str().equivalent(&self.name)
        }
    }
    pub fn find_duplicate<T>(slice: &[T], eq: fn(&T, &T) -> bool) -> Option<(&T, &T)> {
        for (i, first) in slice.iter().enumerate() {
            if let Some(second) = slice[i + 1..].iter().find(|second| eq(first, second))
            {
                return Some((first, second));
            }
        }
        None
    }
    /// Extracts useful info from a `postgres`-generated error.
    pub(crate) fn db_err(
        err: &postgres::Error,
    ) -> Option<(u32, String, Option<String>)> {
        if let Some(db_err) = err.as_db_error() {
            if let Some(ErrorPosition::Original(position)) = db_err.position() {
                Some((
                    *position,
                    db_err.message().to_string(),
                    db_err.hint().map(String::from),
                ))
            } else {
                None
            }
        } else {
            None
        }
    }
    /// Sorted list of rust reserved keywords that cannot be escaped
    pub(crate) const STRICT_KEYWORD: [&str; 5] = ["Self", "_", "crate", "self", "super"];
    /// Sorted list of rust reserved keywords
    pub(crate) const KEYWORD: [&str; 53] = [
        "Self",
        "_",
        "abstract",
        "as",
        "async",
        "await",
        "become",
        "box",
        "break",
        "const",
        "continue",
        "crate",
        "do",
        "dyn",
        "else",
        "enum",
        "extern",
        "false",
        "final",
        "fn",
        "for",
        "if",
        "impl",
        "in",
        "let",
        "loop",
        "macro",
        "match",
        "mod",
        "move",
        "mut",
        "override",
        "priv",
        "pub",
        "ref",
        "return",
        "self",
        "static",
        "struct",
        "super",
        "trait",
        "true",
        "try",
        "type",
        "typeof",
        "union",
        "unsafe",
        "unsized",
        "use",
        "virtual",
        "where",
        "while",
        "yield",
    ];
    /// Escape ident if clash with rust reserved keywords
    pub(crate) fn escape_keyword(ident: String) -> String {
        if KEYWORD.binary_search(&ident.as_str()).is_ok() {
            {
                let res = ::alloc::fmt::format(
                    ::core::fmt::Arguments::new_v1(
                        &["r#"],
                        &[::core::fmt::ArgumentV1::new_display(&ident)],
                    ),
                );
                res
            }
        } else {
            ident
        }
    }
    /// Unescape ident
    pub(crate) fn unescape_keyword(ident: &str) -> &str {
        ident.trim_start_matches("r#")
    }
}
mod validation {
    use std::collections::BTreeMap;
    use crate::{
        parser::{Module, NullableIdent, Query, QueryDataStruct, Span, TypeAnnotation},
        prepare_queries::{PreparedField, PreparedModule},
        read_queries::ModuleInfo,
        utils::{find_duplicate, unescape_keyword, STRICT_KEYWORD},
    };
    use error::Error;
    use heck::ToUpperCamelCase;
    use miette::SourceSpan;
    use postgres::Column;
    use postgres_types::Type;
    pub(crate) fn duplicate_nullable_ident(
        info: &ModuleInfo,
        idents: &[NullableIdent],
    ) -> Result<(), Error> {
        find_duplicate(idents, |a, b| a.name == b.name)
            .map_or(
                Ok(()),
                |(first, second)| {
                    Err(Error::DuplicateFieldNullity {
                        src: info.into(),
                        name: first.name.value.clone(),
                        first: first.name.span,
                        second: second.name.span,
                    })
                },
            )
    }
    pub(crate) fn duplicate_sql_col_name(
        info: &ModuleInfo,
        query_name: &Span<String>,
        cols: &[Column],
    ) -> Result<(), Error> {
        find_duplicate(cols, |a, b| a.name() == b.name())
            .map_or(
                Ok(()),
                |(_, second)| {
                    Err(Error::DuplicateSqlColName {
                        src: info.clone().into(),
                        name: second.name().to_string(),
                        pos: query_name.span,
                    })
                },
            )
    }
    pub(crate) fn query_name_already_used(
        info: &ModuleInfo,
        queries: &[Query],
    ) -> Result<(), Error> {
        find_duplicate(queries, |a, b| a.name == b.name)
            .map_or(
                Ok(()),
                |(first, second)| {
                    Err(Error::DuplicateType {
                        src: info.into(),
                        ty: "query",
                        name: first.name.value.clone(),
                        first: first.name.span,
                        second: second.name.span,
                    })
                },
            )
    }
    pub(crate) fn named_type_already_used(
        info: &ModuleInfo,
        types: &[TypeAnnotation],
    ) -> Result<(), Error> {
        find_duplicate(types, |a, b| a.name == b.name)
            .map_or(
                Ok(()),
                |(first, second)| {
                    Err(Error::DuplicateType {
                        src: info.into(),
                        ty: "type",
                        name: first.name.value.clone(),
                        first: first.name.span,
                        second: second.name.span,
                    })
                },
            )
    }
    pub(crate) fn inline_conflict_declared(
        info: &ModuleInfo,
        name: &Span<String>,
        types: &[TypeAnnotation],
        ty: &'static str,
    ) -> Result<(), Error> {
        if let Some(declared) = types.iter().find(|it| it.name == *name) {
            return Err(Error::DuplicateType {
                src: info.into(),
                ty,
                name: declared.name.value.clone(),
                first: declared.name.span,
                second: name.span,
            });
        }
        Ok(())
    }
    pub(crate) fn reference_unknown_type(
        info: &ModuleInfo,
        name: &Span<String>,
        types: &[TypeAnnotation],
        ty: &'static str,
    ) -> Result<(), Error> {
        if types.iter().all(|it| it.name != *name) {
            return Err(Error::UnknownNamedType {
                src: info.into(),
                ty,
                name: name.value.clone(),
                pos: name.span,
            });
        }
        Ok(())
    }
    pub(crate) fn nullable_column_name(
        info: &ModuleInfo,
        nullable_col: &NullableIdent,
        stmt_cols: &[Column],
    ) -> Result<(), Error> {
        if stmt_cols.iter().all(|row_col| row_col.name() != nullable_col.name.value) {
            return Err(Error::UnknownFieldName {
                src: info.into(),
                pos: nullable_col.name.span,
                known: stmt_cols
                    .iter()
                    .map(|it| it.name().to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }
        Ok(())
    }
    pub(crate) fn nullable_param_name(
        info: &ModuleInfo,
        nullable_col: &NullableIdent,
        params: &[(Span<String>, Type)],
    ) -> Result<(), Error> {
        if params.iter().all(|(name, _)| name.value != nullable_col.name.value) {
            return Err(Error::UnknownFieldName {
                src: info.into(),
                pos: nullable_col.name.span,
                known: params
                    .iter()
                    .map(|it| it.0.value.to_string())
                    .collect::<Vec<_>>()
                    .join(", "),
            });
        }
        Ok(())
    }
    pub(crate) fn row_on_execute(
        info: &ModuleInfo,
        name: &Span<String>,
        query: &SourceSpan,
        row: &QueryDataStruct,
        columns: &[Column],
    ) -> Result<(), Error> {
        if columns.is_empty() && !row.is_empty() {
            return Err(Error::RowOnExecute {
                src: info.into(),
                name: name.value.clone(),
                row: row.span,
                query: *query,
            });
        }
        Ok(())
    }
    pub(crate) fn param_on_simple_query(
        info: &ModuleInfo,
        name: &Span<String>,
        query: &SourceSpan,
        param: &QueryDataStruct,
        fields: &[(Span<String>, Type)],
    ) -> Result<(), Error> {
        if fields.is_empty() && !param.is_empty() {
            return Err(Error::ParamsOnSimpleQuery {
                src: info.into(),
                name: name.value.clone(),
                param: param.span,
                query: *query,
            });
        }
        Ok(())
    }
    fn reserved_type_keyword(info: &ModuleInfo, s: &Span<String>) -> Result<(), Error> {
        if let Ok(it) = STRICT_KEYWORD.binary_search(&s.value.as_str()) {
            return Err(Error::TypeRustKeyword {
                src: info.into(),
                name: STRICT_KEYWORD[it],
                pos: s.span,
            });
        }
        Ok(())
    }
    fn reserved_name_keyword(
        info: &ModuleInfo,
        name: &str,
        pos: &SourceSpan,
        ty: &'static str,
    ) -> Result<(), Error> {
        if let Ok(it) = STRICT_KEYWORD.binary_search(&unescape_keyword(name)) {
            return Err(Error::NameRustKeyword {
                src: info.into(),
                name: STRICT_KEYWORD[it],
                pos: *pos,
                ty,
            });
        }
        Ok(())
    }
    pub(crate) fn named_struct_field(
        info: &ModuleInfo,
        name: &Span<String>,
        fields: &[PreparedField],
        prev_name: &Span<String>,
        prev_fields: &[PreparedField],
    ) -> Result<(), Error> {
        if let Some((field, prev_field))
            = fields
                .iter()
                .find_map(|f| {
                    prev_fields
                        .iter()
                        .find_map(|prev_f| {
                            (f.name == prev_f.name && f.ty != prev_f.ty)
                                .then_some((f, prev_f))
                        })
                })
        {
            return Err(Error::IncompatibleNamedType {
                src: info.into(),
                name: name.value.clone(),
                first_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["column `", "` has type `", "` here"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&field.name),
                                ::core::fmt::ArgumentV1::new_display(&field.ty.pg_ty()),
                            ],
                        ),
                    );
                    res
                },
                second: prev_name.span,
                second_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["but here it has type `", "`"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&prev_field.ty.pg_ty()),
                            ],
                        ),
                    );
                    res
                },
                first: name.span,
            });
        }
        if let Some(field) = fields.iter().find(|f| !prev_fields.contains(f)) {
            return Err(Error::IncompatibleNamedType {
                src: info.into(),
                name: name.value.clone(),
                second_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["column `", "` expected here"],
                            &[::core::fmt::ArgumentV1::new_display(&&field.name)],
                        ),
                    );
                    res
                },
                second: name.span,
                first_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["column `", "` not found"],
                            &[::core::fmt::ArgumentV1::new_display(&&field.name)],
                        ),
                    );
                    res
                },
                first: prev_name.span,
            });
        }
        if let Some(prev_field) = prev_fields.iter().find(|f| !fields.contains(f)) {
            return Err(Error::IncompatibleNamedType {
                src: info.into(),
                name: name.value.clone(),
                second_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["column `", "` expected here"],
                            &[::core::fmt::ArgumentV1::new_display(&&prev_field.name)],
                        ),
                    );
                    res
                },
                second: prev_name.span,
                first_label: {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["column `", "` not found"],
                            &[::core::fmt::ArgumentV1::new_display(&&prev_field.name)],
                        ),
                    );
                    res
                },
                first: name.span,
            });
        }
        Ok(())
    }
    pub(crate) fn validate_preparation(module: &PreparedModule) -> Result<(), Error> {
        let mut name_registrar = BTreeMap::new();
        let mut check_name = |name: String, span: SourceSpan, ty: &'static str| {
            if let Some(prev) = name_registrar.insert(name.clone(), (span, ty)) {
                let (first, second) = if prev.0.offset() < span.offset() {
                    (prev, (span, ty))
                } else {
                    ((span, ty), prev)
                };
                Err(Error::DuplicateName {
                    src: (&module.info).into(),
                    name,
                    first: first.0,
                    first_ty: first.1,
                    second: second.0,
                    second_ty: second.1,
                })
            } else {
                Ok(())
            }
        };
        for (origin, query) in &module.queries {
            reserved_type_keyword(&module.info, origin)?;
            check_name(
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", "Stmt"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &query.name.to_upper_camel_case(),
                                ),
                            ],
                        ),
                    );
                    res
                },
                origin.span,
                "statement",
            )?;
        }
        for (origin, row) in &module.rows {
            reserved_type_keyword(&module.info, origin)?;
            if row.is_named {
                check_name(row.name.value.clone(), origin.span, "row")?;
                for field in &row.fields {
                    reserved_name_keyword(
                        &module.info,
                        &field.name,
                        &origin.span,
                        "row",
                    )?;
                }
                if !row.is_copy {
                    check_name(
                        {
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &["", "Borrowed"],
                                    &[::core::fmt::ArgumentV1::new_display(&row.name)],
                                ),
                            );
                            res
                        },
                        origin.span,
                        "borrowed row",
                    )?;
                }
            }
            check_name(
                {
                    let res = ::alloc::fmt::format(
                        ::core::fmt::Arguments::new_v1(
                            &["", "Query"],
                            &[::core::fmt::ArgumentV1::new_display(&row.name)],
                        ),
                    );
                    res
                },
                origin.span,
                "query",
            )?;
        }
        for (origin, params) in &module.params {
            reserved_type_keyword(&module.info, origin)?;
            if params.is_named {
                check_name(params.name.value.clone(), origin.span, "params")?;
                for field in &params.fields {
                    reserved_name_keyword(
                        &module.info,
                        &field.name,
                        &origin.span,
                        "param",
                    )?;
                }
            }
        }
        Ok(())
    }
    pub(crate) fn validate_module(
        Module { info, types, queries }: &Module,
    ) -> Result<(), Error> {
        query_name_already_used(info, queries)?;
        named_type_already_used(info, types)?;
        for ty in types {
            duplicate_nullable_ident(info, &ty.fields)?;
        }
        for query in queries {
            for (it, ty) in [(&query.param, "param"), (&query.row, "row")] {
                if let Some(idents) = &it.idents {
                    duplicate_nullable_ident(info, idents)?;
                }
                if let Some(name) = &it.name {
                    if it.inlined() {
                        inline_conflict_declared(info, name, types, ty)?;
                    } else {
                        reference_unknown_type(info, name, types, ty)?;
                    }
                }
            }
        }
        Ok(())
    }
    pub mod error {
        use std::fmt::Debug;
        use miette::{Diagnostic, NamedSource, SourceSpan};
        use thiserror::Error as ThisError;
        pub enum Error {
            #[error("column `{name}` appear multiple time")]
            #[diagnostic(
                help("disambiguate column names in your SQL using an `AS` clause")
            )]
            DuplicateSqlColName {
                #[source_code]
                src: NamedSource,
                name: String,
                #[label("query returns one or more columns with the same name")]
                pos: SourceSpan,
            },
            #[error("the field `{name}` is declared null multiple time")]
            #[diagnostic(help("remove one of the two declaration"))]
            DuplicateFieldNullity {
                #[source_code]
                src: NamedSource,
                name: String,
                #[label("previous nullity declaration")]
                first: SourceSpan,
                #[label("redeclared here")]
                second: SourceSpan,
            },
            #[error("the {ty} `{name}` is defined multiple time")]
            #[diagnostic(help("use a different name for one of those"))]
            DuplicateType {
                #[source_code]
                src: NamedSource,
                name: String,
                ty: &'static str,
                #[label("previous definition here")]
                first: SourceSpan,
                #[label("redefined here")]
                second: SourceSpan,
            },
            #[error("reference to an unknown named {ty} `{name}`")]
            #[diagnostic(help("declare an inline named type using `()`: {name}()"))]
            UnknownNamedType {
                #[source_code]
                src: NamedSource,
                name: String,
                ty: &'static str,
                #[label("unknown named {ty}")]
                pos: SourceSpan,
            },
            #[error("unknown field")]
            #[diagnostic(help("use one of those names: {known}"))]
            UnknownFieldName {
                #[source_code]
                src: NamedSource,
                #[label("no field with this name was found")]
                pos: SourceSpan,
                known: String,
            },
            #[error("named type `{name}` as conflicting usage")]
            #[diagnostic(help("use a different named type for each query"))]
            IncompatibleNamedType {
                #[source_code]
                src: NamedSource,
                name: String,
                first_label: String,
                #[label("{first_label}")]
                first: SourceSpan,
                second_label: String,
                #[label("{second_label}")]
                second: SourceSpan,
            },
            #[error("the query `{name}` declare a row but return nothing")]
            #[diagnostic(help("remove row declaration"))]
            RowOnExecute {
                #[source_code]
                src: NamedSource,
                name: String,
                #[label("row declared here")]
                row: SourceSpan,
                #[label("but query return nothing")]
                query: SourceSpan,
            },
            #[error("the query `{name}` declares a parameter but has no binding")]
            #[diagnostic(help("remove parameter declaration"))]
            ParamsOnSimpleQuery {
                #[source_code]
                src: NamedSource,
                name: String,
                #[label("parameter declared here")]
                param: SourceSpan,
                #[label("but query has no binding")]
                query: SourceSpan,
            },
            #[error("`{name}` is used multiple time")]
            #[diagnostic(help("use a different name for one of those"))]
            DuplicateName {
                #[source_code]
                src: NamedSource,
                name: String,
                first_ty: &'static str,
                #[label("previous definition as {first_ty} here")]
                first: SourceSpan,
                second_ty: &'static str,
                #[label("redefined as {second_ty} here")]
                second: SourceSpan,
            },
            #[error("`{name}` is a reserved rust keyword that cannot be escaped")]
            #[diagnostic(help("use a different name"))]
            TypeRustKeyword {
                #[source_code]
                src: NamedSource,
                name: &'static str,
                #[label("reserved rust keyword")]
                pos: SourceSpan,
            },
            #[error("`{name}` is a reserved rust keyword that cannot be escaped")]
            #[diagnostic(help("use a different name"))]
            NameRustKeyword {
                #[source_code]
                src: NamedSource,
                name: &'static str,
                ty: &'static str,
                #[label("from {ty} declared here")]
                pos: SourceSpan,
            },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Error::DuplicateSqlColName {
                        src: __self_0,
                        name: __self_1,
                        pos: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "DuplicateSqlColName",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "pos",
                            &__self_2,
                        )
                    }
                    Error::DuplicateFieldNullity {
                        src: __self_0,
                        name: __self_1,
                        first: __self_2,
                        second: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "DuplicateFieldNullity",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "first",
                            &__self_2,
                            "second",
                            &__self_3,
                        )
                    }
                    Error::DuplicateType {
                        src: __self_0,
                        name: __self_1,
                        ty: __self_2,
                        first: __self_3,
                        second: __self_4,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field5_finish(
                            f,
                            "DuplicateType",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "ty",
                            &__self_2,
                            "first",
                            &__self_3,
                            "second",
                            &__self_4,
                        )
                    }
                    Error::UnknownNamedType {
                        src: __self_0,
                        name: __self_1,
                        ty: __self_2,
                        pos: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "UnknownNamedType",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "ty",
                            &__self_2,
                            "pos",
                            &__self_3,
                        )
                    }
                    Error::UnknownFieldName {
                        src: __self_0,
                        pos: __self_1,
                        known: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "UnknownFieldName",
                            "src",
                            &__self_0,
                            "pos",
                            &__self_1,
                            "known",
                            &__self_2,
                        )
                    }
                    Error::IncompatibleNamedType {
                        src: __self_0,
                        name: __self_1,
                        first_label: __self_2,
                        first: __self_3,
                        second_label: __self_4,
                        second: __self_5,
                    } => {
                        let names: &'static _ = &[
                            "src",
                            "name",
                            "first_label",
                            "first",
                            "second_label",
                            "second",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &__self_0,
                            &__self_1,
                            &__self_2,
                            &__self_3,
                            &__self_4,
                            &__self_5,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "IncompatibleNamedType",
                            names,
                            values,
                        )
                    }
                    Error::RowOnExecute {
                        src: __self_0,
                        name: __self_1,
                        row: __self_2,
                        query: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "RowOnExecute",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "row",
                            &__self_2,
                            "query",
                            &__self_3,
                        )
                    }
                    Error::ParamsOnSimpleQuery {
                        src: __self_0,
                        name: __self_1,
                        param: __self_2,
                        query: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "ParamsOnSimpleQuery",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "param",
                            &__self_2,
                            "query",
                            &__self_3,
                        )
                    }
                    Error::DuplicateName {
                        src: __self_0,
                        name: __self_1,
                        first_ty: __self_2,
                        first: __self_3,
                        second_ty: __self_4,
                        second: __self_5,
                    } => {
                        let names: &'static _ = &[
                            "src",
                            "name",
                            "first_ty",
                            "first",
                            "second_ty",
                            "second",
                        ];
                        let values: &[&dyn ::core::fmt::Debug] = &[
                            &__self_0,
                            &__self_1,
                            &__self_2,
                            &__self_3,
                            &__self_4,
                            &__self_5,
                        ];
                        ::core::fmt::Formatter::debug_struct_fields_finish(
                            f,
                            "DuplicateName",
                            names,
                            values,
                        )
                    }
                    Error::TypeRustKeyword {
                        src: __self_0,
                        name: __self_1,
                        pos: __self_2,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "TypeRustKeyword",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "pos",
                            &__self_2,
                        )
                    }
                    Error::NameRustKeyword {
                        src: __self_0,
                        name: __self_1,
                        ty: __self_2,
                        pos: __self_3,
                    } => {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "NameRustKeyword",
                            "src",
                            &__self_0,
                            "name",
                            &__self_1,
                            "ty",
                            &__self_2,
                            "pos",
                            &__self_3,
                        )
                    }
                }
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated, clippy::used_underscore_binding)]
                match self {
                    Error::DuplicateSqlColName { src, name, pos } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["column `", "` appear multiple time"],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::DuplicateFieldNullity { src, name, first, second } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["the field `", "` is declared null multiple time"],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::DuplicateType { src, name, ty, first, second } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["the ", " `", "` is defined multiple time"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&ty.as_display()),
                                        ::core::fmt::ArgumentV1::new_display(&name.as_display()),
                                    ],
                                ),
                            )
                    }
                    Error::UnknownNamedType { src, name, ty, pos } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["reference to an unknown named ", " `", "`"],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&ty.as_display()),
                                        ::core::fmt::ArgumentV1::new_display(&name.as_display()),
                                    ],
                                ),
                            )
                    }
                    Error::UnknownFieldName { src, pos, known } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(&["unknown field"], &[]),
                            )
                    }
                    Error::IncompatibleNamedType {
                        src,
                        name,
                        first_label,
                        first,
                        second_label,
                        second,
                    } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["named type `", "` as conflicting usage"],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::RowOnExecute { src, name, row, query } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["the query `", "` declare a row but return nothing"],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::ParamsOnSimpleQuery { src, name, param, query } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "the query `",
                                        "` declares a parameter but has no binding",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::DuplicateName {
                        src,
                        name,
                        first_ty,
                        first,
                        second_ty,
                        second,
                    } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &["`", "` is used multiple time"],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::TypeRustKeyword { src, name, pos } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "`",
                                        "` is a reserved rust keyword that cannot be escaped",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                    Error::NameRustKeyword { src, name, ty, pos } => {
                        __formatter
                            .write_fmt(
                                ::core::fmt::Arguments::new_v1(
                                    &[
                                        "`",
                                        "` is a reserved rust keyword that cannot be escaped",
                                    ],
                                    &[::core::fmt::ArgumentV1::new_display(&name.as_display())],
                                ),
                            )
                    }
                }
            }
        }
        impl miette::Diagnostic for Error {
            fn help<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::DuplicateSqlColName { src, name, pos } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "disambiguate column names in your SQL using an `AS` clause",
                                        ],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::DuplicateFieldNullity { src, name, first, second } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["remove one of the two declaration"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::DuplicateType { src, name, ty, first, second } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use a different name for one of those"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::UnknownNamedType { src, name, ty, pos } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["declare an inline named type using `()`: ", "()"],
                                        &[::core::fmt::ArgumentV1::new_display(&name)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::UnknownFieldName { src, pos, known } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use one of those names: "],
                                        &[::core::fmt::ArgumentV1::new_display(&known)],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::IncompatibleNamedType {
                        src,
                        name,
                        first_label,
                        first,
                        second_label,
                        second,
                    } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use a different named type for each query"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::RowOnExecute { src, name, row, query } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["remove row declaration"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::ParamsOnSimpleQuery { src, name, param, query } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["remove parameter declaration"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::DuplicateName {
                        src,
                        name,
                        first_ty,
                        first,
                        second_ty,
                        second,
                    } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use a different name for one of those"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::TypeRustKeyword { src, name, pos } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use a different name"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    Self::NameRustKeyword { src, name, ty, pos } => {
                        std::option::Option::Some(
                            std::boxed::Box::new({
                                let res = ::alloc::fmt::format(
                                    ::core::fmt::Arguments::new_v1(
                                        &["use a different name"],
                                        &[],
                                    ),
                                );
                                res
                            }),
                        )
                    }
                    _ => std::option::Option::None,
                }
            }
            fn labels(
                &self,
            ) -> std::option::Option<
                std::boxed::Box<dyn std::iter::Iterator<Item = miette::LabeledSpan> + '_>,
            > {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::DuplicateSqlColName { src, name, pos } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(pos)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["query returns one or more columns with the same name"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::DuplicateFieldNullity { src, name, first, second } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(first)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["previous nullity declaration"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(second)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(&["redeclared here"], &[]),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::DuplicateType { src, name, ty, first, second } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(first)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["previous definition here"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(second)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(&["redefined here"], &[]),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::UnknownNamedType { src, name, ty, pos } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(pos)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["unknown named "],
                                                                &[::core::fmt::ArgumentV1::new_display(&ty)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::UnknownFieldName { src, pos, known } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(pos)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["no field with this name was found"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::IncompatibleNamedType {
                        src,
                        name,
                        first_label,
                        first,
                        second_label,
                        second,
                    } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(first)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &[""],
                                                                &[::core::fmt::ArgumentV1::new_display(&first_label)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(second)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &[""],
                                                                &[::core::fmt::ArgumentV1::new_display(&second_label)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::RowOnExecute { src, name, row, query } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(row)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(&["row declared here"], &[]),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(query)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["but query return nothing"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::ParamsOnSimpleQuery { src, name, param, query } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(param)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["parameter declared here"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(query)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["but query has no binding"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::DuplicateName {
                        src,
                        name,
                        first_ty,
                        first,
                        second_ty,
                        second,
                    } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(first)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["previous definition as ", " here"],
                                                                &[::core::fmt::ArgumentV1::new_display(&first_ty)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(second)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["redefined as ", " here"],
                                                                &[::core::fmt::ArgumentV1::new_display(&second_ty)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::TypeRustKeyword { src, name, pos } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(pos)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["reserved rust keyword"],
                                                                &[],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    Self::NameRustKeyword { src, name, ty, pos } => {
                        use miette::macro_helpers::ToOption;
                        std::option::Option::Some(
                            std::boxed::Box::new(
                                <[_]>::into_vec(
                                        #[rustc_box]
                                        ::alloc::boxed::Box::new([
                                            miette::macro_helpers::OptionalWrapper::<SourceSpan>::new()
                                                .to_option(pos)
                                                .map(|__miette_internal_var| miette::LabeledSpan::new_with_span(
                                                    std::option::Option::Some({
                                                        let res = ::alloc::fmt::format(
                                                            ::core::fmt::Arguments::new_v1(
                                                                &["from ", " declared here"],
                                                                &[::core::fmt::ArgumentV1::new_display(&ty)],
                                                            ),
                                                        );
                                                        res
                                                    }),
                                                    __miette_internal_var.clone(),
                                                )),
                                        ]),
                                    )
                                    .into_iter()
                                    .filter(Option::is_some)
                                    .map(Option::unwrap),
                            ),
                        )
                    }
                    _ => std::option::Option::None,
                }
            }
            fn source_code(&self) -> std::option::Option<&dyn miette::SourceCode> {
                #[allow(unused_variables, deprecated)]
                match self {
                    Self::DuplicateSqlColName { src, name, pos } => {
                        std::option::Option::Some(src)
                    }
                    Self::DuplicateFieldNullity { src, name, first, second } => {
                        std::option::Option::Some(src)
                    }
                    Self::DuplicateType { src, name, ty, first, second } => {
                        std::option::Option::Some(src)
                    }
                    Self::UnknownNamedType { src, name, ty, pos } => {
                        std::option::Option::Some(src)
                    }
                    Self::UnknownFieldName { src, pos, known } => {
                        std::option::Option::Some(src)
                    }
                    Self::IncompatibleNamedType {
                        src,
                        name,
                        first_label,
                        first,
                        second_label,
                        second,
                    } => std::option::Option::Some(src),
                    Self::RowOnExecute { src, name, row, query } => {
                        std::option::Option::Some(src)
                    }
                    Self::ParamsOnSimpleQuery { src, name, param, query } => {
                        std::option::Option::Some(src)
                    }
                    Self::DuplicateName {
                        src,
                        name,
                        first_ty,
                        first,
                        second_ty,
                        second,
                    } => std::option::Option::Some(src),
                    Self::TypeRustKeyword { src, name, pos } => {
                        std::option::Option::Some(src)
                    }
                    Self::NameRustKeyword { src, name, ty, pos } => {
                        std::option::Option::Some(src)
                    }
                    _ => std::option::Option::None,
                }
            }
        }
    }
}
/// Helpers to establish connections to database instances.
pub mod conn {
    use postgres::{Client, Config, NoTls};
    use self::error::Error;
    /// Creates a non-TLS connection from a URL.
    pub(crate) fn from_url(url: &str) -> Result<Client, Error> {
        Ok(Client::connect(url, NoTls)?)
    }
    /// Create a non-TLS connection to the container managed by Cornucopia.
    pub fn cornucopia_conn() -> Result<Client, Error> {
        Ok(
            Config::new()
                .user("postgres")
                .password("postgres")
                .host("127.0.0.1")
                .port(5435)
                .dbname("postgres")
                .connect(NoTls)?,
        )
    }
    pub(crate) mod error {
        use miette::Diagnostic;
        #[error("Couldn't establish a connection with the database.")]
        pub struct Error(#[from] pub postgres::Error);
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Error", &&self.0)
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {
            fn source(&self) -> std::option::Option<&(dyn std::error::Error + 'static)> {
                use thiserror::__private::AsDynError;
                std::option::Option::Some(self.0.as_dyn_error())
            }
        }
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_variables, deprecated)]
                let Self(_0) = self;
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["Couldn\'t establish a connection with the database."],
                            &[],
                        ),
                    )
            }
        }
        #[allow(unused_qualifications)]
        impl std::convert::From<postgres::Error> for Error {
            #[allow(deprecated)]
            fn from(source: postgres::Error) -> Self {
                Error { 0: source }
            }
        }
        impl miette::Diagnostic for Error {}
    }
}
/// High-level interfaces to work with Cornucopia's container manager.
pub mod container {
    use std::process::{Command, Stdio};
    use self::error::Error;
    /// Starts Cornucopia's database container and wait until it reports healthy.
    pub fn setup(podman: bool) -> Result<(), Error> {
        spawn_container(podman)?;
        healthcheck(podman, 120, 50)?;
        Ok(())
    }
    /// Stop and remove a container and its volume.
    pub fn cleanup(podman: bool) -> Result<(), Error> {
        stop_container(podman)?;
        remove_container(podman)?;
        Ok(())
    }
    /// Starts Cornucopia's database container.
    fn spawn_container(podman: bool) -> Result<(), Error> {
        cmd(
            podman,
            &[
                "run",
                "-d",
                "--name",
                "cornucopia_postgres",
                "-p",
                "5435:5432",
                "-e",
                "POSTGRES_PASSWORD=postgres",
                "postgres",
            ],
            "spawn container",
        )
    }
    /// Checks if Cornucopia's container reports healthy
    fn is_postgres_healthy(podman: bool) -> Result<bool, Error> {
        Ok(
            cmd(
                    podman,
                    &["exec", "cornucopia_postgres", "pg_isready"],
                    "check container health",
                )
                .is_ok(),
        )
    }
    /// This function controls how the healthcheck retries are handled.
    fn healthcheck(
        podman: bool,
        max_retries: u64,
        ms_per_retry: u64,
    ) -> Result<(), Error> {
        let slow_threshold = 10 + max_retries / 10;
        let mut nb_retries = 0;
        while !is_postgres_healthy(podman)? {
            if nb_retries >= max_retries {
                return Err(
                    Error::new(
                        String::from(
                            "Cornucopia reached the max number of connection retries",
                        ),
                        podman,
                    ),
                );
            }
            std::thread::sleep(std::time::Duration::from_millis(ms_per_retry));
            nb_retries += 1;
            if nb_retries % slow_threshold == 0 {
                {
                    ::std::io::_print(
                        ::core::fmt::Arguments::new_v1(
                            &[
                                "Container startup slower than expected (",
                                " retries out of ",
                                ")\n",
                            ],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&nb_retries),
                                ::core::fmt::ArgumentV1::new_display(&max_retries),
                            ],
                        ),
                    );
                };
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        Ok(())
    }
    /// Stops Cornucopia's container.
    fn stop_container(podman: bool) -> Result<(), Error> {
        cmd(podman, &["stop", "cornucopia_postgres"], "stop container")
    }
    /// Removes Cornucopia's container and its volume.
    fn remove_container(podman: bool) -> Result<(), Error> {
        cmd(podman, &["rm", "-v", "cornucopia_postgres"], "remove container")
    }
    fn cmd(
        podman: bool,
        args: &[&'static str],
        action: &'static str,
    ) -> Result<(), Error> {
        let command = if podman { "podman" } else { "docker" };
        let output = Command::new(command)
            .args(args)
            .stderr(Stdio::piped())
            .stdout(Stdio::null())
            .output()?;
        if output.status.success() {
            Ok(())
        } else {
            let err = String::from_utf8_lossy(&output.stderr);
            Err(
                Error::new(
                    {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1(
                                &["`", "` couldn\'t ", ": "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(&command),
                                    ::core::fmt::ArgumentV1::new_display(&action),
                                    ::core::fmt::ArgumentV1::new_display(&err),
                                ],
                            ),
                        );
                        res
                    },
                    podman,
                ),
            )
        }
    }
    pub(crate) mod error {
        use std::fmt::Debug;
        use miette::Diagnostic;
        use thiserror::Error as ThisError;
        #[error("{msg}")]
        pub struct Error {
            msg: String,
            #[help]
            pub help: Option<String>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Error {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Error",
                    "msg",
                    &&self.msg,
                    "help",
                    &&self.help,
                )
            }
        }
        #[allow(unused_qualifications)]
        impl std::error::Error for Error {}
        #[allow(unused_qualifications)]
        impl std::fmt::Display for Error {
            #[allow(clippy::used_underscore_binding)]
            fn fmt(&self, __formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                #[allow(unused_imports)]
                use thiserror::__private::{DisplayAsDisplay, PathAsDisplay};
                #[allow(unused_variables, deprecated)]
                let Self { msg, help } = self;
                __formatter
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &[""],
                            &[::core::fmt::ArgumentV1::new_display(&msg.as_display())],
                        ),
                    )
            }
        }
        impl miette::Diagnostic for Error {
            fn help<'a>(
                &'a self,
            ) -> std::option::Option<std::boxed::Box<dyn std::fmt::Display + 'a>> {
                #[allow(unused_variables, deprecated)]
                let Self { msg, help } = self;
                use miette::macro_helpers::ToOption;
                miette::macro_helpers::OptionalWrapper::<Option<String>>::new()
                    .to_option(&self.help)
                    .as_ref()
                    .map(|
                        __miette_internal_var,
                    | -> std::boxed::Box<dyn std::fmt::Display + 'a> {
                        std::boxed::Box::new({
                            let res = ::alloc::fmt::format(
                                ::core::fmt::Arguments::new_v1(
                                    &[""],
                                    &[
                                        ::core::fmt::ArgumentV1::new_display(&__miette_internal_var),
                                    ],
                                ),
                            );
                            res
                        })
                    })
            }
        }
        impl Error {
            pub fn new(msg: String, podman: bool) -> Self {
                let help = if podman {
                    "Make sure that port 5435 is usable and that no container named `cornucopia_postgres` already exists."
                } else {
                    "First, check that the docker daemon is up-and-running. Then, make sure that port 5435 is usable and that no container named `cornucopia_postgres` already exists."
                };
                Error {
                    msg,
                    help: Some(String::from(help)),
                }
            }
        }
        impl From<std::io::Error> for Error {
            fn from(e: std::io::Error) -> Self {
                Self {
                    msg: {
                        let res = ::alloc::fmt::format(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &[""],
                                &[::core::fmt::ArgumentV1::new_display(&e)],
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                        res
                    },
                    help: None,
                }
            }
        }
    }
}
use postgres::Client;
use codegen::generate as generate_internal;
use error::WriteOutputError;
use parser::parse_query_module;
use prepare_queries::prepare;
use read_queries::read_query_modules;
#[doc(hidden)]
pub use cli::run;
pub use error::Error;
pub use load_schema::load_schema;
/// Struct containing the settings for code generation.
pub struct CodegenSettings {
    pub is_async: bool,
    pub derive_ser: bool,
}
#[automatically_derived]
impl ::core::clone::Clone for CodegenSettings {
    #[inline]
    fn clone(&self) -> CodegenSettings {
        let _: ::core::clone::AssertParamIsClone<bool>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for CodegenSettings {}
/// Generates Rust queries from PostgreSQL queries located at `queries_path`,
/// using a live database managed by you. If some `destination` is given,
/// the generated code will be written at that path. Code generation settings are
/// set using the `settings` parameter.
pub fn generate_live(
    client: &mut Client,
    queries_path: &str,
    destination: Option<&str>,
    settings: CodegenSettings,
) -> Result<String, Error> {
    let modules = read_query_modules(queries_path)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;
    let prepared_modules = prepare(client, modules)?;
    let generated_code = generate_internal(prepared_modules, settings);
    if let Some(d) = destination {
        write_generated_code(d, &generated_code)?;
    }
    Ok(generated_code)
}
/// Generates Rust queries from PostgreSQL queries located at `queries_path`, using
/// a container managed by cornucopia. The database schema is created using `schema_files`.
/// If some `destination` is given, the generated code will be written at that path.
/// Code generation settings are set using the `settings` parameter.
///
/// By default, the container manager is Docker, but Podman can be used by setting the
/// `podman` parameter to `true`.
pub fn generate_managed(
    queries_path: &str,
    schema_files: Vec<String>,
    destination: Option<&str>,
    podman: bool,
    settings: CodegenSettings,
) -> Result<String, Error> {
    let modules = read_query_modules(queries_path)?
        .into_iter()
        .map(parse_query_module)
        .collect::<Result<_, parser::error::Error>>()?;
    container::setup(podman)?;
    let mut client = conn::cornucopia_conn()?;
    load_schema(&mut client, schema_files)?;
    let prepared_modules = prepare(&mut client, modules)?;
    let generated_code = generate_internal(prepared_modules, settings);
    container::cleanup(podman)?;
    if let Some(destination) = destination {
        write_generated_code(destination, &generated_code)?;
    }
    Ok(generated_code)
}
fn write_generated_code(destination: &str, generated_code: &str) -> Result<(), Error> {
    Ok(
        std::fs::write(destination, generated_code)
            .map_err(|err| WriteOutputError {
                err,
                file_path: String::from(destination),
            })?,
    )
}
