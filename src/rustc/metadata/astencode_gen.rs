/*syntax::ast::ident*/
fn serialize_1<S: std::serialization::serializer>(s: S,
                                                  v: syntax::ast::ident) {

    s.emit_str(v);
}
/*syntax::ast::attr_style*/
fn serialize_5<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      syntax::ast::attr_style) {
    s.emit_enum("syntax::ast::attr_style",

                {||
                    alt v {
                      syntax::ast::attr_outer {
                        s.emit_enum_variant("syntax::ast::attr_outer", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::attr_inner {
                        s.emit_enum_variant("syntax::ast::attr_inner", 1u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*@syntax::ast::meta_item*/
fn serialize_9<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      @syntax::ast::meta_item) {
    s.emit_box(/*syntax::ast::meta_item*/{|| serialize_6(s, *v) });
}
/*[@syntax::ast::meta_item]*/
fn serialize_8<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      [@syntax::ast::meta_item]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::meta_item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_9(s, e) })
                              })
               });
}
/*str*/
fn serialize_12<S: std::serialization::serializer>(s: S, v: str) {

    s.emit_str(v);
}
/*i64*/
fn serialize_13<S: std::serialization::serializer>(s: S, v: i64) {

    s.emit_i64(v);
}
/*syntax::ast::int_ty*/
fn serialize_14<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::int_ty) {

    s.emit_enum("syntax::ast::int_ty",





                {||
                    alt v {
                      syntax::ast::ty_i {
                        s.emit_enum_variant("syntax::ast::ty_i", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_char {
                        s.emit_enum_variant("syntax::ast::ty_char", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_i8 {
                        s.emit_enum_variant("syntax::ast::ty_i8", 2u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_i16 {
                        s.emit_enum_variant("syntax::ast::ty_i16", 3u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_i32 {
                        s.emit_enum_variant("syntax::ast::ty_i32", 4u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_i64 {
                        s.emit_enum_variant("syntax::ast::ty_i64", 5u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*u64*/
fn serialize_15<S: std::serialization::serializer>(s: S, v: u64) {

    s.emit_u64(v);
}
/*syntax::ast::uint_ty*/
fn serialize_16<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::uint_ty) {

    s.emit_enum("syntax::ast::uint_ty",




                {||
                    alt v {
                      syntax::ast::ty_u {
                        s.emit_enum_variant("syntax::ast::ty_u", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_u8 {
                        s.emit_enum_variant("syntax::ast::ty_u8", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_u16 {
                        s.emit_enum_variant("syntax::ast::ty_u16", 2u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_u32 {
                        s.emit_enum_variant("syntax::ast::ty_u32", 3u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_u64 {
                        s.emit_enum_variant("syntax::ast::ty_u64", 4u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::float_ty*/
fn serialize_17<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::float_ty) {

    s.emit_enum("syntax::ast::float_ty",


                {||
                    alt v {
                      syntax::ast::ty_f {
                        s.emit_enum_variant("syntax::ast::ty_f", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_f32 {
                        s.emit_enum_variant("syntax::ast::ty_f32", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_f64 {
                        s.emit_enum_variant("syntax::ast::ty_f64", 2u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*bool*/
fn serialize_18<S: std::serialization::serializer>(s: S, v: bool) {

    s.emit_bool(v);
}
/*syntax::ast::lit_*/
fn serialize_11<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::lit_) {

    s.emit_enum("syntax::ast::lit_",
                /*str*/
                /*i64*//*syntax::ast::int_ty*/
                /*u64*//*syntax::ast::uint_ty*/
                /*str*//*syntax::ast::float_ty*/

                /*bool*/
                {||
                    alt v {
                      syntax::ast::lit_str(v0) {
                        s.emit_enum_variant("syntax::ast::lit_str", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_12(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::lit_int(v0, v1) {
                        s.emit_enum_variant("syntax::ast::lit_int", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_13(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_14(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::lit_uint(v0, v1) {
                        s.emit_enum_variant("syntax::ast::lit_uint", 2u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_15(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_16(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::lit_float(v0, v1) {
                        s.emit_enum_variant("syntax::ast::lit_float", 3u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_12(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_17(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::lit_nil {
                        s.emit_enum_variant("syntax::ast::lit_nil", 4u, 0u,
                                            {|| })
                      }
                      syntax::ast::lit_bool(v0) {
                        s.emit_enum_variant("syntax::ast::lit_bool", 5u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_18(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*uint*/
fn serialize_20<S: std::serialization::serializer>(s: S, v: uint) {

    s.emit_uint(v);
}
/*core::option::t<syntax::codemap::span>*/
fn serialize_26<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       core::option::t<syntax::codemap::span>) {
    s.emit_enum("core::option::t",

                /*syntax::codemap::span*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_19(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*{name: str,span: core::option::t<syntax::codemap::span>}*/
fn serialize_25<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       {name: str,
                                                        span:
                                                            core::option::t<syntax::codemap::span>,}) {
    s.emit_rec(/*str*//*core::option::t<syntax::codemap::span>*/
               {||
                   {
                       s.emit_rec_field("name", 0u,
                                        {|| serialize_12(s, v.name) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_26(s, v.span) })
                   }
               });
}

/*{call_site: syntax::codemap::span,callie: {name: str,span: core::option::t<syntax::codemap::span>}}*/
fn serialize_24<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       {call_site:
                                                            syntax::codemap::span,
                                                        callie:
                                                            {name: str,
                                                             span:
                                                                 core::option::t<syntax::codemap::span>,},}) {
    s.emit_rec(/*syntax::codemap::span*/
               /*{name: str,span: core::option::t<syntax::codemap::span>}*/
               {||
                   {
                       s.emit_rec_field("call_site", 0u,
                                        {|| serialize_19(s, v.call_site) });
                       s.emit_rec_field("callie", 1u,
                                        {|| serialize_25(s, v.callie) })
                   }
               });
}
/*syntax::codemap::expn_info_*/
fn serialize_23<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::codemap::expn_info_) {
    s.emit_enum("syntax::codemap::expn_info_",

                /*{call_site: syntax::codemap::span,callie: {name: str,span: core::option::t<syntax::codemap::span>}}*/
                {||
                    alt v {
                      syntax::codemap::expanded_from(v0) {
                        s.emit_enum_variant("syntax::codemap::expanded_from",
                                            0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_24(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*@syntax::codemap::expn_info_*/
fn serialize_22<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @syntax::codemap::expn_info_) {
    s.emit_box(/*syntax::codemap::expn_info_*/{|| serialize_23(s, *v) });
}
/*syntax::codemap::expn_info<@syntax::codemap::expn_info_>*/
fn serialize_21<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::codemap::expn_info<@syntax::codemap::expn_info_>) {
    s.emit_enum("core::option::t",

                /*@syntax::codemap::expn_info_*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_22(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::codemap::span*/
fn serialize_19<S: std::serialization::serializer>(s: S,
                                                   v: syntax::codemap::span) {

    s.emit_rec(/*uint*//*uint*/
               /*syntax::codemap::expn_info<@syntax::codemap::expn_info_>*/
               {||
                   {
                       s.emit_rec_field("lo", 0u,
                                        {|| serialize_20(s, v.lo) });
                       s.emit_rec_field("hi", 1u,
                                        {|| serialize_20(s, v.hi) });
                       s.emit_rec_field("expn_info", 2u,
                                        {|| serialize_21(s, v.expn_info) })
                   }
               });
}
/*syntax::ast::lit*/
fn serialize_10<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::lit) {

    s.emit_rec(/*syntax::ast::lit_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_11(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::meta_item_*/
fn serialize_7<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      syntax::ast::meta_item_) {
    s.emit_enum("syntax::ast::meta_item_",
                /*syntax::ast::ident*/
                /*syntax::ast::ident*//*[@syntax::ast::meta_item]*/
                /*syntax::ast::ident*//*syntax::ast::lit*/
                {||
                    alt v {
                      syntax::ast::meta_word(v0) {
                        s.emit_enum_variant("syntax::ast::meta_word", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::meta_list(v0, v1) {
                        s.emit_enum_variant("syntax::ast::meta_list", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_8(s,
                                                                                            v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::meta_name_value(v0, v1) {
                        s.emit_enum_variant("syntax::ast::meta_name_value",
                                            2u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_10(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::meta_item*/
fn serialize_6<S: std::serialization::serializer>(s: S,
                                                  v: syntax::ast::meta_item) {

    s.emit_rec(/*syntax::ast::meta_item_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_7(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::attribute_*/
fn serialize_4<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      syntax::ast::attribute_) {
    s.emit_rec(/*syntax::ast::attr_style*//*syntax::ast::meta_item*/
               {||
                   {
                       s.emit_rec_field("style", 0u,
                                        {|| serialize_5(s, v.style) });
                       s.emit_rec_field("value", 1u,
                                        {|| serialize_6(s, v.value) })
                   }
               });
}
/*syntax::ast::attribute*/
fn serialize_3<S: std::serialization::serializer>(s: S,
                                                  v: syntax::ast::attribute) {

    s.emit_rec(/*syntax::ast::attribute_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_4(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::attribute]*/
fn serialize_2<S: std::serialization::serializer>(s: S,
                                                  v:
                                                      [syntax::ast::attribute]) {
    s.emit_vec(vec::len(v), /*syntax::ast::attribute*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_3(s, e) })
                              })
               });
}
/*syntax::ast::node_id*/
fn serialize_27<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::node_id) {

    s.emit_int(v);
}
/*syntax::ast::mutability*/
fn serialize_33<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::mutability) {
    s.emit_enum("syntax::ast::mutability",


                {||
                    alt v {
                      syntax::ast::m_mutbl {
                        s.emit_enum_variant("syntax::ast::m_mutbl", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::m_imm {
                        s.emit_enum_variant("syntax::ast::m_imm", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::m_const {
                        s.emit_enum_variant("syntax::ast::m_const", 2u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::mt*/
fn serialize_32<S: std::serialization::serializer>(s: S, v: syntax::ast::mt) {

    s.emit_rec(/*@syntax::ast::ty*//*syntax::ast::mutability*/
               {||
                   {
                       s.emit_rec_field("ty", 0u,
                                        {|| serialize_29(s, v.ty) });
                       s.emit_rec_field("mutbl", 1u,
                                        {|| serialize_33(s, v.mutbl) })
                   }
               });
}
/*syntax::ast::ty_field_*/
fn serialize_36<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::ty_field_) {
    s.emit_rec(/*syntax::ast::ident*//*syntax::ast::mt*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("mt", 1u, {|| serialize_32(s, v.mt) })
                   }
               });
}
/*syntax::ast::ty_field*/
fn serialize_35<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::ty_field) {

    s.emit_rec(/*syntax::ast::ty_field_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_36(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::ty_field]*/
fn serialize_34<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [syntax::ast::ty_field]) {
    s.emit_vec(vec::len(v), /*syntax::ast::ty_field*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_35(s, e) })
                              })
               });
}
/*syntax::ast::proto*/
fn serialize_37<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::proto) {

    s.emit_enum("syntax::ast::proto",




                {||
                    alt v {
                      syntax::ast::proto_bare {
                        s.emit_enum_variant("syntax::ast::proto_bare", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::proto_any {
                        s.emit_enum_variant("syntax::ast::proto_any", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::proto_uniq {
                        s.emit_enum_variant("syntax::ast::proto_uniq", 2u, 0u,
                                            {|| })
                      }
                      syntax::ast::proto_box {
                        s.emit_enum_variant("syntax::ast::proto_box", 3u, 0u,
                                            {|| })
                      }
                      syntax::ast::proto_block {
                        s.emit_enum_variant("syntax::ast::proto_block", 4u,
                                            0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::rmode*/
fn serialize_42<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::rmode) {

    s.emit_enum("syntax::ast::rmode",




                {||
                    alt v {
                      syntax::ast::by_ref {
                        s.emit_enum_variant("syntax::ast::by_ref", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::by_val {
                        s.emit_enum_variant("syntax::ast::by_val", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::by_mutbl_ref {
                        s.emit_enum_variant("syntax::ast::by_mutbl_ref", 2u,
                                            0u, {|| })
                      }
                      syntax::ast::by_move {
                        s.emit_enum_variant("syntax::ast::by_move", 3u, 0u,
                                            {|| })
                      }
                      syntax::ast::by_copy {
                        s.emit_enum_variant("syntax::ast::by_copy", 4u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::mode<syntax::ast::rmode>*/
fn serialize_41<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::mode<syntax::ast::rmode>) {
    s.emit_enum("syntax::ast::inferable",
                /*syntax::ast::rmode*/
                /*syntax::ast::node_id*/
                {||
                    alt v {
                      syntax::ast::expl(v0) {
                        s.emit_enum_variant("syntax::ast::expl", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_42(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::infer(v0) {
                        s.emit_enum_variant("syntax::ast::infer", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::arg*/
fn serialize_40<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::arg) {

    s.emit_rec(/*syntax::ast::mode<syntax::ast::rmode>*//*@syntax::ast::ty*/
               /*syntax::ast::ident*//*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("mode", 0u,
                                        {|| serialize_41(s, v.mode) });
                       s.emit_rec_field("ty", 1u,
                                        {|| serialize_29(s, v.ty) });
                       s.emit_rec_field("ident", 2u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("id", 3u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*[syntax::ast::arg]*/
fn serialize_39<S: std::serialization::serializer>(s: S,
                                                   v: [syntax::ast::arg]) {

    s.emit_vec(vec::len(v), /*syntax::ast::arg*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_40(s, e) })
                              })
               });
}
/*syntax::ast::purity*/
fn serialize_43<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::purity) {

    s.emit_enum("syntax::ast::purity",



                {||
                    alt v {
                      syntax::ast::pure_fn {
                        s.emit_enum_variant("syntax::ast::pure_fn", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::unsafe_fn {
                        s.emit_enum_variant("syntax::ast::unsafe_fn", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::impure_fn {
                        s.emit_enum_variant("syntax::ast::impure_fn", 2u, 0u,
                                            {|| })
                      }
                      syntax::ast::crust_fn {
                        s.emit_enum_variant("syntax::ast::crust_fn", 3u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::ret_style*/
fn serialize_44<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::ret_style) {
    s.emit_enum("syntax::ast::ret_style",

                {||
                    alt v {
                      syntax::ast::noreturn {
                        s.emit_enum_variant("syntax::ast::noreturn", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::return_val {
                        s.emit_enum_variant("syntax::ast::return_val", 1u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*[syntax::ast::ident]*/
fn serialize_52<S: std::serialization::serializer>(s: S,
                                                   v: [syntax::ast::ident]) {

    s.emit_vec(vec::len(v), /*syntax::ast::ident*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_1(s, e) })
                              })
               });
}
/*[@syntax::ast::ty]*/
fn serialize_53<S: std::serialization::serializer>(s: S,
                                                   v: [@syntax::ast::ty]) {

    s.emit_vec(vec::len(v), /*@syntax::ast::ty*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_29(s, e) })
                              })
               });
}
/*syntax::ast::path_*/
fn serialize_51<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::path_) {

    s.emit_rec(/*bool*//*[syntax::ast::ident]*//*[@syntax::ast::ty]*/
               {||
                   {
                       s.emit_rec_field("global", 0u,
                                        {|| serialize_18(s, v.global) });
                       s.emit_rec_field("idents", 1u,
                                        {|| serialize_52(s, v.idents) });
                       s.emit_rec_field("types", 2u,
                                        {|| serialize_53(s, v.types) })
                   }
               });
}
/*syntax::ast::path*/
fn serialize_50<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::path) {

    s.emit_rec(/*syntax::ast::path_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_51(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::path*/
fn serialize_49<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::path) {

    s.emit_box(/*syntax::ast::path*/{|| serialize_50(s, *v) });
}
/*@syntax::ast::lit*/
fn serialize_58<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::lit) {

    s.emit_box(/*syntax::ast::lit*/{|| serialize_10(s, *v) });
}
/*syntax::ast::constr_arg_general_<uint>*/
fn serialize_57<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::constr_arg_general_<uint>) {
    s.emit_enum("syntax::ast::constr_arg_general_",

                /*uint*/
                /*@syntax::ast::lit*/
                {||
                    alt v {
                      syntax::ast::carg_base {
                        s.emit_enum_variant("syntax::ast::carg_base", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::carg_ident(v0) {
                        s.emit_enum_variant("syntax::ast::carg_ident", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::carg_lit(v0) {
                        s.emit_enum_variant("syntax::ast::carg_lit", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_58(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
fn serialize_56<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       {node:
                                                            syntax::ast::constr_arg_general_<uint>,
                                                        span:
                                                            syntax::codemap::span,}) {
    s.emit_rec(/*syntax::ast::constr_arg_general_<uint>*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_57(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}

/*@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
fn serialize_55<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @{node:
                                                             syntax::ast::constr_arg_general_<uint>,
                                                         span:
                                                             syntax::codemap::span,}) {
    s.emit_box(
               /*{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
               {|| serialize_56(s, *v) });
}

/*[@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}]*/
fn serialize_54<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@{node:
                                                              syntax::ast::constr_arg_general_<uint>,
                                                          span:
                                                              syntax::codemap::span,}]) {
    s.emit_vec(vec::len(v),
               /*@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_55(s, e) })
                              })
               });
}

/*{path: @syntax::ast::path,args: [@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}],id: syntax::ast::node_id}*/
fn serialize_48<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       {path:
                                                            @syntax::ast::path,
                                                        args:
                                                            [@{node:
                                                                   syntax::ast::constr_arg_general_<uint>,
                                                               span:
                                                                   syntax::codemap::span,}],
                                                        id:
                                                            syntax::ast::node_id,}) {
    s.emit_rec(/*@syntax::ast::path*/
               /*[@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}]*/
               /*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("path", 0u,
                                        {|| serialize_49(s, v.path) });
                       s.emit_rec_field("args", 1u,
                                        {|| serialize_54(s, v.args) });
                       s.emit_rec_field("id", 2u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*syntax::ast::constr*/
fn serialize_47<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::constr) {

    s.emit_rec(
               /*{path: @syntax::ast::path,args: [@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}],id: syntax::ast::node_id}*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_48(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::constr*/
fn serialize_46<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::constr) {

    s.emit_box(/*syntax::ast::constr*/{|| serialize_47(s, *v) });
}
/*[@syntax::ast::constr]*/
fn serialize_45<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@syntax::ast::constr]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::constr*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_46(s, e) })
                              })
               });
}
/*syntax::ast::fn_decl*/
fn serialize_38<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::fn_decl) {

    s.emit_rec(/*[syntax::ast::arg]*//*@syntax::ast::ty*/
               /*syntax::ast::purity*//*syntax::ast::ret_style*/
               /*[@syntax::ast::constr]*/
               {||
                   {
                       s.emit_rec_field("inputs", 0u,
                                        {|| serialize_39(s, v.inputs) });
                       s.emit_rec_field("output", 1u,
                                        {|| serialize_29(s, v.output) });
                       s.emit_rec_field("purity", 2u,
                                        {|| serialize_43(s, v.purity) });
                       s.emit_rec_field("cf", 3u,
                                        {|| serialize_44(s, v.cf) });
                       s.emit_rec_field("constraints", 4u,
                                        {|| serialize_45(s, v.constraints) })
                   }
               });
}
/*syntax::ast::constr_arg_general_<@syntax::ast::path>*/
fn serialize_66<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::constr_arg_general_<@syntax::ast::path>) {
    s.emit_enum("syntax::ast::constr_arg_general_",

                /*@syntax::ast::path*/
                /*@syntax::ast::lit*/
                {||
                    alt v {
                      syntax::ast::carg_base {
                        s.emit_enum_variant("syntax::ast::carg_base", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::carg_ident(v0) {
                        s.emit_enum_variant("syntax::ast::carg_ident", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::carg_lit(v0) {
                        s.emit_enum_variant("syntax::ast::carg_lit", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_58(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}

/*{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
fn serialize_65<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       {node:
                                                            syntax::ast::constr_arg_general_<@syntax::ast::path>,
                                                        span:
                                                            syntax::codemap::span,}) {
    s.emit_rec(/*syntax::ast::constr_arg_general_<@syntax::ast::path>*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_66(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}

/*@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
fn serialize_64<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @{node:
                                                             syntax::ast::constr_arg_general_<@syntax::ast::path>,
                                                         span:
                                                             syntax::codemap::span,}) {
    s.emit_box(
               /*{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
               {|| serialize_65(s, *v) });
}

/*[@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}]*/
fn serialize_63<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@{node:
                                                              syntax::ast::constr_arg_general_<@syntax::ast::path>,
                                                          span:
                                                              syntax::codemap::span,}]) {
    s.emit_vec(vec::len(v),
               /*@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_64(s, e) })
                              })
               });
}
/*syntax::ast::ty_constr_*/
fn serialize_62<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::ty_constr_) {
    s.emit_rec(/*@syntax::ast::path*/
               /*[@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}]*/
               /*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("path", 0u,
                                        {|| serialize_49(s, v.path) });
                       s.emit_rec_field("args", 1u,
                                        {|| serialize_63(s, v.args) });
                       s.emit_rec_field("id", 2u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*syntax::ast::ty_constr*/
fn serialize_61<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::ty_constr) {
    s.emit_rec(/*syntax::ast::ty_constr_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_62(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::ty_constr*/
fn serialize_60<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @syntax::ast::ty_constr) {
    s.emit_box(/*syntax::ast::ty_constr*/{|| serialize_61(s, *v) });
}
/*[@syntax::ast::ty_constr]*/
fn serialize_59<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@syntax::ast::ty_constr]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::ty_constr*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_60(s, e) })
                              })
               });
}
/*[@syntax::ast::expr]*/
fn serialize_73<S: std::serialization::serializer>(s: S,
                                                   v: [@syntax::ast::expr]) {

    s.emit_vec(vec::len(v), /*@syntax::ast::expr*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_70(s, e) })
                              })
               });
}
/*syntax::ast::field_*/
fn serialize_76<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::field_) {

    s.emit_rec(/*syntax::ast::mutability*//*syntax::ast::ident*/
               /*@syntax::ast::expr*/
               {||
                   {
                       s.emit_rec_field("mutbl", 0u,
                                        {|| serialize_33(s, v.mutbl) });
                       s.emit_rec_field("ident", 1u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("expr", 2u,
                                        {|| serialize_70(s, v.expr) })
                   }
               });
}
/*syntax::ast::field*/
fn serialize_75<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::field) {

    s.emit_rec(/*syntax::ast::field_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_76(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::field]*/
fn serialize_74<S: std::serialization::serializer>(s: S,
                                                   v: [syntax::ast::field]) {

    s.emit_vec(vec::len(v), /*syntax::ast::field*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_75(s, e) })
                              })
               });
}
/*core::option::t<@syntax::ast::expr>*/
fn serialize_77<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       core::option::t<@syntax::ast::expr>) {
    s.emit_enum("core::option::t",

                /*@syntax::ast::expr*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*[core::option::t<@syntax::ast::expr>]*/
fn serialize_78<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [core::option::t<@syntax::ast::expr>]) {
    s.emit_vec(vec::len(v), /*core::option::t<@syntax::ast::expr>*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_77(s, e) })
                              })
               });
}
/*syntax::ast::binop*/
fn serialize_79<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::binop) {

    s.emit_enum("syntax::ast::binop",


















                {||
                    alt v {
                      syntax::ast::add {
                        s.emit_enum_variant("syntax::ast::add", 0u, 0u, {|| })
                      }
                      syntax::ast::subtract {
                        s.emit_enum_variant("syntax::ast::subtract", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::mul {
                        s.emit_enum_variant("syntax::ast::mul", 2u, 0u, {|| })
                      }
                      syntax::ast::div {
                        s.emit_enum_variant("syntax::ast::div", 3u, 0u, {|| })
                      }
                      syntax::ast::rem {
                        s.emit_enum_variant("syntax::ast::rem", 4u, 0u, {|| })
                      }
                      syntax::ast::and {
                        s.emit_enum_variant("syntax::ast::and", 5u, 0u, {|| })
                      }
                      syntax::ast::or {
                        s.emit_enum_variant("syntax::ast::or", 6u, 0u, {|| })
                      }
                      syntax::ast::bitxor {
                        s.emit_enum_variant("syntax::ast::bitxor", 7u, 0u,
                                            {|| })
                      }
                      syntax::ast::bitand {
                        s.emit_enum_variant("syntax::ast::bitand", 8u, 0u,
                                            {|| })
                      }
                      syntax::ast::bitor {
                        s.emit_enum_variant("syntax::ast::bitor", 9u, 0u,
                                            {|| })
                      }
                      syntax::ast::lsl {
                        s.emit_enum_variant("syntax::ast::lsl", 10u, 0u,
                                            {|| })
                      }
                      syntax::ast::lsr {
                        s.emit_enum_variant("syntax::ast::lsr", 11u, 0u,
                                            {|| })
                      }
                      syntax::ast::asr {
                        s.emit_enum_variant("syntax::ast::asr", 12u, 0u,
                                            {|| })
                      }
                      syntax::ast::eq {
                        s.emit_enum_variant("syntax::ast::eq", 13u, 0u, {|| })
                      }
                      syntax::ast::lt {
                        s.emit_enum_variant("syntax::ast::lt", 14u, 0u, {|| })
                      }
                      syntax::ast::le {
                        s.emit_enum_variant("syntax::ast::le", 15u, 0u, {|| })
                      }
                      syntax::ast::ne {
                        s.emit_enum_variant("syntax::ast::ne", 16u, 0u, {|| })
                      }
                      syntax::ast::ge {
                        s.emit_enum_variant("syntax::ast::ge", 17u, 0u, {|| })
                      }
                      syntax::ast::gt {
                        s.emit_enum_variant("syntax::ast::gt", 18u, 0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::unop*/
fn serialize_80<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::unop) {

    s.emit_enum("syntax::ast::unop",
                /*syntax::ast::mutability*/
                /*syntax::ast::mutability*/


                {||
                    alt v {
                      syntax::ast::box(v0) {
                        s.emit_enum_variant("syntax::ast::box", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_33(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::uniq(v0) {
                        s.emit_enum_variant("syntax::ast::uniq", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_33(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::deref {
                        s.emit_enum_variant("syntax::ast::deref", 2u, 0u,
                                            {|| })
                      }
                      syntax::ast::not {
                        s.emit_enum_variant("syntax::ast::not", 3u, 0u, {|| })
                      }
                      syntax::ast::neg {
                        s.emit_enum_variant("syntax::ast::neg", 4u, 0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::simple_path*/
fn serialize_92<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::simple_path) {
    s.emit_vec(vec::len(v), /*syntax::ast::ident*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_1(s, e) })
                              })
               });
}
/*@syntax::ast::simple_path*/
fn serialize_91<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @syntax::ast::simple_path) {
    s.emit_box(/*syntax::ast::simple_path*/{|| serialize_92(s, *v) });
}
/*syntax::ast::path_list_ident_*/
fn serialize_95<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::path_list_ident_) {
    s.emit_rec(/*syntax::ast::ident*//*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("name", 0u,
                                        {|| serialize_1(s, v.name) });
                       s.emit_rec_field("id", 1u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*syntax::ast::path_list_ident*/
fn serialize_94<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::path_list_ident) {
    s.emit_rec(/*syntax::ast::path_list_ident_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_95(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::path_list_ident]*/
fn serialize_93<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [syntax::ast::path_list_ident]) {
    s.emit_vec(vec::len(v), /*syntax::ast::path_list_ident*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_94(s, e) })
                              })
               });
}
/*syntax::ast::view_path_*/
fn serialize_90<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::view_path_) {
    s.emit_enum("syntax::ast::view_path_",
                /*syntax::ast::ident*//*@syntax::ast::simple_path*/
                /*syntax::ast::node_id*/
                /*@syntax::ast::simple_path*//*syntax::ast::node_id*/
                /*@syntax::ast::simple_path*/
                /*[syntax::ast::path_list_ident]*//*syntax::ast::node_id*/
                {||
                    alt v {
                      syntax::ast::view_path_simple(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::view_path_simple",
                                            0u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_91(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::view_path_glob(v0, v1) {
                        s.emit_enum_variant("syntax::ast::view_path_glob", 1u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_91(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::view_path_list(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::view_path_list", 2u,
                                            3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_91(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_93(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::view_path*/
fn serialize_89<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::view_path) {
    s.emit_rec(/*syntax::ast::view_path_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_90(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::view_path*/
fn serialize_88<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @syntax::ast::view_path) {
    s.emit_box(/*syntax::ast::view_path*/{|| serialize_89(s, *v) });
}
/*[@syntax::ast::view_path]*/
fn serialize_87<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@syntax::ast::view_path]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::view_path*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_88(s, e) })
                              })
               });
}
/*syntax::ast::view_item_*/
fn serialize_86<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::view_item_) {
    s.emit_enum("syntax::ast::view_item_",
                /*syntax::ast::ident*//*[@syntax::ast::meta_item]*/
                /*syntax::ast::node_id*/
                /*[@syntax::ast::view_path]*/
                /*[@syntax::ast::view_path]*/
                {||
                    alt v {
                      syntax::ast::view_item_use(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::view_item_use", 0u,
                                            3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_8(s,
                                                                                            v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::view_item_import(v0) {
                        s.emit_enum_variant("syntax::ast::view_item_import",
                                            1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_87(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::view_item_export(v0) {
                        s.emit_enum_variant("syntax::ast::view_item_export",
                                            2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_87(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::view_item*/
fn serialize_85<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::view_item) {
    s.emit_rec(/*syntax::ast::view_item_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_86(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::view_item*/
fn serialize_84<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       @syntax::ast::view_item) {
    s.emit_box(/*syntax::ast::view_item*/{|| serialize_85(s, *v) });
}
/*[@syntax::ast::view_item]*/
fn serialize_83<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       [@syntax::ast::view_item]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::view_item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_84(s, e) })
                              })
               });
}
/*core::option::t<@syntax::ast::pat>*/
fn serialize_110<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        core::option::t<@syntax::ast::pat>) {
    s.emit_enum("core::option::t",

                /*@syntax::ast::pat*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_107(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*[@syntax::ast::pat]*/
fn serialize_111<S: std::serialization::serializer>(s: S,
                                                    v: [@syntax::ast::pat]) {

    s.emit_vec(vec::len(v), /*@syntax::ast::pat*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_107(s, e) })
                              })
               });
}
/*syntax::ast::field_pat*/
fn serialize_113<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::field_pat) {
    s.emit_rec(/*syntax::ast::ident*//*@syntax::ast::pat*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("pat", 1u,
                                        {|| serialize_107(s, v.pat) })
                   }
               });
}
/*[syntax::ast::field_pat]*/
fn serialize_112<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::field_pat]) {
    s.emit_vec(vec::len(v), /*syntax::ast::field_pat*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_113(s, e) })
                              })
               });
}
/*syntax::ast::pat_*/
fn serialize_109<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::pat_) {

    s.emit_enum("syntax::ast::pat_",

                /*@syntax::ast::path*//*core::option::t<@syntax::ast::pat>*/
                /*@syntax::ast::path*//*[@syntax::ast::pat]*/
                /*[syntax::ast::field_pat]*//*bool*/
                /*[@syntax::ast::pat]*/
                /*@syntax::ast::pat*/
                /*@syntax::ast::pat*/
                /*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                {||
                    alt v {
                      syntax::ast::pat_wild {
                        s.emit_enum_variant("syntax::ast::pat_wild", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::pat_ident(v0, v1) {
                        s.emit_enum_variant("syntax::ast::pat_ident", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_110(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_enum(v0, v1) {
                        s.emit_enum_variant("syntax::ast::pat_enum", 2u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_111(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_rec(v0, v1) {
                        s.emit_enum_variant("syntax::ast::pat_rec", 3u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_112(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_18(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_tup(v0) {
                        s.emit_enum_variant("syntax::ast::pat_tup", 4u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_111(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_box(v0) {
                        s.emit_enum_variant("syntax::ast::pat_box", 5u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_107(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_uniq(v0) {
                        s.emit_enum_variant("syntax::ast::pat_uniq", 6u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_107(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_lit(v0) {
                        s.emit_enum_variant("syntax::ast::pat_lit", 7u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::pat_range(v0, v1) {
                        s.emit_enum_variant("syntax::ast::pat_range", 8u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::pat*/
fn serialize_108<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::pat) {

    s.emit_rec(/*syntax::ast::node_id*//*syntax::ast::pat_*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("id", 0u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("node", 1u,
                                        {|| serialize_109(s, v.node) });
                       s.emit_rec_field("span", 2u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::pat*/
fn serialize_107<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::pat) {

    s.emit_box(/*syntax::ast::pat*/{|| serialize_108(s, *v) });
}
/*syntax::ast::init_op*/
fn serialize_116<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::init_op) {

    s.emit_enum("syntax::ast::init_op",

                {||
                    alt v {
                      syntax::ast::init_assign {
                        s.emit_enum_variant("syntax::ast::init_assign", 0u,
                                            0u, {|| })
                      }
                      syntax::ast::init_move {
                        s.emit_enum_variant("syntax::ast::init_move", 1u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::initializer*/
fn serialize_115<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::initializer) {
    s.emit_rec(/*syntax::ast::init_op*//*@syntax::ast::expr*/
               {||
                   {
                       s.emit_rec_field("op", 0u,
                                        {|| serialize_116(s, v.op) });
                       s.emit_rec_field("expr", 1u,
                                        {|| serialize_70(s, v.expr) })
                   }
               });
}
/*core::option::t<syntax::ast::initializer>*/
fn serialize_114<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        core::option::t<syntax::ast::initializer>) {
    s.emit_enum("core::option::t",

                /*syntax::ast::initializer*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_115(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::local_*/
fn serialize_106<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::local_) {

    s.emit_rec(/*bool*//*@syntax::ast::ty*//*@syntax::ast::pat*/
               /*core::option::t<syntax::ast::initializer>*/
               /*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("is_mutbl", 0u,
                                        {|| serialize_18(s, v.is_mutbl) });
                       s.emit_rec_field("ty", 1u,
                                        {|| serialize_29(s, v.ty) });
                       s.emit_rec_field("pat", 2u,
                                        {|| serialize_107(s, v.pat) });
                       s.emit_rec_field("init", 3u,
                                        {|| serialize_114(s, v.init) });
                       s.emit_rec_field("id", 4u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*syntax::ast::local*/
fn serialize_105<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::local) {

    s.emit_rec(/*syntax::ast::local_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_106(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::local*/
fn serialize_104<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::local) {

    s.emit_box(/*syntax::ast::local*/{|| serialize_105(s, *v) });
}
/*[@syntax::ast::local]*/
fn serialize_103<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [@syntax::ast::local]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::local*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_104(s, e) })
                              })
               });
}
/*@syntax::ast::item*/
fn serialize_117<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::item) {

    s.emit_box(/*syntax::ast::item*/{|| serialize_0(s, *v) });
}
/*syntax::ast::decl_*/
fn serialize_102<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::decl_) {

    s.emit_enum("syntax::ast::decl_",
                /*[@syntax::ast::local]*/
                /*@syntax::ast::item*/
                {||
                    alt v {
                      syntax::ast::decl_local(v0) {
                        s.emit_enum_variant("syntax::ast::decl_local", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_103(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::decl_item(v0) {
                        s.emit_enum_variant("syntax::ast::decl_item", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_117(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::decl*/
fn serialize_101<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::decl) {

    s.emit_rec(/*syntax::ast::decl_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_102(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::decl*/
fn serialize_100<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::decl) {

    s.emit_box(/*syntax::ast::decl*/{|| serialize_101(s, *v) });
}
/*syntax::ast::stmt_*/
fn serialize_99<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::stmt_) {

    s.emit_enum("syntax::ast::stmt_",
                /*@syntax::ast::decl*//*syntax::ast::node_id*/
                /*@syntax::ast::expr*//*syntax::ast::node_id*/
                /*@syntax::ast::expr*//*syntax::ast::node_id*/
                {||
                    alt v {
                      syntax::ast::stmt_decl(v0, v1) {
                        s.emit_enum_variant("syntax::ast::stmt_decl", 0u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_100(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::stmt_expr(v0, v1) {
                        s.emit_enum_variant("syntax::ast::stmt_expr", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::stmt_semi(v0, v1) {
                        s.emit_enum_variant("syntax::ast::stmt_semi", 2u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::stmt*/
fn serialize_98<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::stmt) {

    s.emit_rec(/*syntax::ast::stmt_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_99(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::stmt*/
fn serialize_97<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::stmt) {

    s.emit_box(/*syntax::ast::stmt*/{|| serialize_98(s, *v) });
}
/*[@syntax::ast::stmt]*/
fn serialize_96<S: std::serialization::serializer>(s: S,
                                                   v: [@syntax::ast::stmt]) {

    s.emit_vec(vec::len(v), /*@syntax::ast::stmt*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_97(s, e) })
                              })
               });
}
/*syntax::ast::blk_check_mode*/
fn serialize_118<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::blk_check_mode) {
    s.emit_enum("syntax::ast::blk_check_mode",


                {||
                    alt v {
                      syntax::ast::default_blk {
                        s.emit_enum_variant("syntax::ast::default_blk", 0u,
                                            0u, {|| })
                      }
                      syntax::ast::unchecked_blk {
                        s.emit_enum_variant("syntax::ast::unchecked_blk", 1u,
                                            0u, {|| })
                      }
                      syntax::ast::unsafe_blk {
                        s.emit_enum_variant("syntax::ast::unsafe_blk", 2u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::blk_*/
fn serialize_82<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::blk_) {

    s.emit_rec(/*[@syntax::ast::view_item]*//*[@syntax::ast::stmt]*/
               /*core::option::t<@syntax::ast::expr>*//*syntax::ast::node_id*/
               /*syntax::ast::blk_check_mode*/
               {||
                   {
                       s.emit_rec_field("view_items", 0u,
                                        {|| serialize_83(s, v.view_items) });
                       s.emit_rec_field("stmts", 1u,
                                        {|| serialize_96(s, v.stmts) });
                       s.emit_rec_field("expr", 2u,
                                        {|| serialize_77(s, v.expr) });
                       s.emit_rec_field("id", 3u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("rules", 4u,
                                        {|| serialize_118(s, v.rules) })
                   }
               });
}
/*syntax::ast::blk*/
fn serialize_81<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::blk) {

    s.emit_rec(/*syntax::ast::blk_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_82(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::arm*/
fn serialize_120<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::arm) {

    s.emit_rec(/*[@syntax::ast::pat]*//*core::option::t<@syntax::ast::expr>*/
               /*syntax::ast::blk*/
               {||
                   {
                       s.emit_rec_field("pats", 0u,
                                        {|| serialize_111(s, v.pats) });
                       s.emit_rec_field("guard", 1u,
                                        {|| serialize_77(s, v.guard) });
                       s.emit_rec_field("body", 2u,
                                        {|| serialize_81(s, v.body) })
                   }
               });
}
/*[syntax::ast::arm]*/
fn serialize_119<S: std::serialization::serializer>(s: S,
                                                    v: [syntax::ast::arm]) {

    s.emit_vec(vec::len(v), /*syntax::ast::arm*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_120(s, e) })
                              })
               });
}
/*syntax::ast::alt_mode*/
fn serialize_121<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::alt_mode) {
    s.emit_enum("syntax::ast::alt_mode",

                {||
                    alt v {
                      syntax::ast::alt_check {
                        s.emit_enum_variant("syntax::ast::alt_check", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::alt_exhaustive {
                        s.emit_enum_variant("syntax::ast::alt_exhaustive", 1u,
                                            0u, {|| })
                      }
                    }
                });
}
/*int*/
fn serialize_127<S: std::serialization::serializer>(s: S, v: int) {

    s.emit_int(v);
}
/*syntax::ast::capture_item*/
fn serialize_126<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::capture_item) {
    s.emit_rec(/*int*//*syntax::ast::ident*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("id", 0u,
                                        {|| serialize_127(s, v.id) });
                       s.emit_rec_field("name", 1u,
                                        {|| serialize_1(s, v.name) });
                       s.emit_rec_field("span", 2u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::capture_item*/
fn serialize_125<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        @syntax::ast::capture_item) {
    s.emit_box(/*syntax::ast::capture_item*/{|| serialize_126(s, *v) });
}
/*[@syntax::ast::capture_item]*/
fn serialize_124<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [@syntax::ast::capture_item]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::capture_item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_125(s, e) })
                              })
               });
}
/*syntax::ast::capture_clause*/
fn serialize_123<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::capture_clause) {
    s.emit_rec(/*[@syntax::ast::capture_item]*/
               /*[@syntax::ast::capture_item]*/
               {||
                   {
                       s.emit_rec_field("copies", 0u,
                                        {|| serialize_124(s, v.copies) });
                       s.emit_rec_field("moves", 1u,
                                        {|| serialize_124(s, v.moves) })
                   }
               });
}
/*@syntax::ast::capture_clause*/
fn serialize_122<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        @syntax::ast::capture_clause) {
    s.emit_box(/*syntax::ast::capture_clause*/{|| serialize_123(s, *v) });
}
/*syntax::ast::expr_check_mode*/
fn serialize_128<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::expr_check_mode) {
    s.emit_enum("syntax::ast::expr_check_mode",

                {||
                    alt v {
                      syntax::ast::claimed_expr {
                        s.emit_enum_variant("syntax::ast::claimed_expr", 0u,
                                            0u, {|| })
                      }
                      syntax::ast::checked_expr {
                        s.emit_enum_variant("syntax::ast::checked_expr", 1u,
                                            0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::expr_*/
fn serialize_72<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::expr_) {

    s.emit_enum("syntax::ast::expr_",
                /*[@syntax::ast::expr]*//*syntax::ast::mutability*/
                /*[syntax::ast::field]*/
                /*core::option::t<@syntax::ast::expr>*/
                /*@syntax::ast::expr*//*[@syntax::ast::expr]*//*bool*/
                /*[@syntax::ast::expr]*/
                /*@syntax::ast::expr*/
                /*[core::option::t<@syntax::ast::expr>]*/
                /*syntax::ast::binop*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*/
                /*syntax::ast::unop*//*@syntax::ast::expr*/
                /*@syntax::ast::lit*/
                /*@syntax::ast::expr*//*@syntax::ast::ty*/
                /*@syntax::ast::expr*//*syntax::ast::blk*/
                /*core::option::t<@syntax::ast::expr>*/
                /*@syntax::ast::expr*//*syntax::ast::blk*/
                /*@syntax::ast::local*//*@syntax::ast::expr*/
                /*syntax::ast::blk*/
                /*syntax::ast::blk*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*[syntax::ast::arm]*/
                /*syntax::ast::alt_mode*/
                /*syntax::ast::proto*//*syntax::ast::fn_decl*/
                /*syntax::ast::blk*//*@syntax::ast::capture_clause*/
                /*syntax::ast::fn_decl*//*syntax::ast::blk*/
                /*syntax::ast::blk*/
                /*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                /*syntax::ast::binop*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*syntax::ast::ident*/
                /*[@syntax::ast::ty]*/
                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                /*@syntax::ast::path*/
                /*core::option::t<@syntax::ast::expr>*/


                /*core::option::t<@syntax::ast::expr>*/
                /*@syntax::ast::expr*/
                /*int*//*@syntax::ast::expr*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*/
                /*syntax::ast::expr_check_mode*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*//*syntax::ast::blk*/
                /*core::option::t<@syntax::ast::expr>*/
                /*syntax::ast::mac*/
                {||
                    alt v {
                      syntax::ast::expr_vec(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_vec", 0u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_73(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_33(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_rec(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_rec", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_74(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_77(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_call(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_call", 2u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_73(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_18(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_tup(v0) {
                        s.emit_enum_variant("syntax::ast::expr_tup", 3u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_73(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_bind(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_bind", 4u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_78(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_binary(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_binary", 5u,
                                            3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_79(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_unary(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_unary", 6u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_80(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_lit(v0) {
                        s.emit_enum_variant("syntax::ast::expr_lit", 7u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_58(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_cast(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_cast", 8u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_if(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_if", 9u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_77(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_while(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_while", 10u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_for(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_for", 11u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_104(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_do_while(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_do_while", 12u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_alt(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_alt", 13u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_119(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_121(s,
                                                                                              v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_fn(v0, v1, v2, v3) {
                        s.emit_enum_variant("syntax::ast::expr_fn", 14u, 4u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_37(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v2)
                                                                            });
                                                    s.emit_enum_variant_arg(3u,
                                                                            {||
                                                                                serialize_122(s,
                                                                                              v3)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_fn_block(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_fn_block", 15u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_block(v0) {
                        s.emit_enum_variant("syntax::ast::expr_block", 16u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_copy(v0) {
                        s.emit_enum_variant("syntax::ast::expr_copy", 17u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_move(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_move", 18u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_assign(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_assign", 19u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_swap(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_swap", 20u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_assign_op(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_assign_op",
                                            21u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_79(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_field(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_field", 22u,
                                            3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_53(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_index(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_index", 23u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_path(v0) {
                        s.emit_enum_variant("syntax::ast::expr_path", 24u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_fail(v0) {
                        s.emit_enum_variant("syntax::ast::expr_fail", 25u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_77(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_break {
                        s.emit_enum_variant("syntax::ast::expr_break", 26u,
                                            0u, {|| })
                      }
                      syntax::ast::expr_cont {
                        s.emit_enum_variant("syntax::ast::expr_cont", 27u, 0u,
                                            {|| })
                      }
                      syntax::ast::expr_ret(v0) {
                        s.emit_enum_variant("syntax::ast::expr_ret", 28u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_77(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_be(v0) {
                        s.emit_enum_variant("syntax::ast::expr_be", 29u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_log(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_log", 30u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_127(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_assert(v0) {
                        s.emit_enum_variant("syntax::ast::expr_assert", 31u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_check(v0, v1) {
                        s.emit_enum_variant("syntax::ast::expr_check", 32u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_128(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_if_check(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::expr_if_check", 33u,
                                            3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_77(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::expr_mac(v0) {
                        s.emit_enum_variant("syntax::ast::expr_mac", 34u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_67(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::expr*/
fn serialize_71<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::expr) {

    s.emit_rec(/*syntax::ast::node_id*//*syntax::ast::expr_*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("id", 0u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("node", 1u,
                                        {|| serialize_72(s, v.node) });
                       s.emit_rec_field("span", 2u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::expr*/
fn serialize_70<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::expr) {

    s.emit_box(/*syntax::ast::expr*/{|| serialize_71(s, *v) });
}
/*syntax::ast::mac_arg<@syntax::ast::expr>*/
fn serialize_69<S: std::serialization::serializer>(s: S,
                                                   v:
                                                       syntax::ast::mac_arg<@syntax::ast::expr>) {
    s.emit_enum("core::option::t",

                /*@syntax::ast::expr*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::mac_body_*/
fn serialize_130<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::mac_body_) {
    s.emit_rec(/*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("span", 0u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::mac_body<syntax::ast::mac_body_>*/
fn serialize_129<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::mac_body<syntax::ast::mac_body_>) {
    s.emit_enum("core::option::t",

                /*syntax::ast::mac_body_*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_130(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::mac_*/
fn serialize_68<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::mac_) {

    s.emit_enum("syntax::ast::mac_",
                /*@syntax::ast::path*/
                /*syntax::ast::mac_arg<@syntax::ast::expr>*/
                /*syntax::ast::mac_body<syntax::ast::mac_body_>*/
                /*@syntax::ast::ty*/
                /*syntax::ast::blk*/

                /*syntax::codemap::span*//*@syntax::ast::expr*/
                /*uint*/
                {||
                    alt v {
                      syntax::ast::mac_invoc(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::mac_invoc", 0u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_69(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_129(s,
                                                                                              v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::mac_embed_type(v0) {
                        s.emit_enum_variant("syntax::ast::mac_embed_type", 1u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::mac_embed_block(v0) {
                        s.emit_enum_variant("syntax::ast::mac_embed_block",
                                            2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::mac_ellipsis {
                        s.emit_enum_variant("syntax::ast::mac_ellipsis", 3u,
                                            0u, {|| })
                      }
                      syntax::ast::mac_aq(v0, v1) {
                        s.emit_enum_variant("syntax::ast::mac_aq", 4u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_19(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::mac_var(v0) {
                        s.emit_enum_variant("syntax::ast::mac_var", 5u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::mac*/
fn serialize_67<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::mac) {

    s.emit_rec(/*syntax::ast::mac_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_68(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::ty_*/
fn serialize_31<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::ty_) {

    s.emit_enum("syntax::ast::ty_",


                /*syntax::ast::mt*/
                /*syntax::ast::mt*/
                /*syntax::ast::mt*/
                /*syntax::ast::mt*/
                /*[syntax::ast::ty_field]*/
                /*syntax::ast::proto*//*syntax::ast::fn_decl*/
                /*[@syntax::ast::ty]*/
                /*@syntax::ast::path*//*syntax::ast::node_id*/
                /*@syntax::ast::ty*//*[@syntax::ast::ty_constr]*/
                /*syntax::ast::mac*/
                {||
                    alt v {
                      syntax::ast::ty_nil {
                        s.emit_enum_variant("syntax::ast::ty_nil", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_bot {
                        s.emit_enum_variant("syntax::ast::ty_bot", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_box(v0) {
                        s.emit_enum_variant("syntax::ast::ty_box", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_32(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_uniq(v0) {
                        s.emit_enum_variant("syntax::ast::ty_uniq", 3u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_32(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_vec(v0) {
                        s.emit_enum_variant("syntax::ast::ty_vec", 4u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_32(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_ptr(v0) {
                        s.emit_enum_variant("syntax::ast::ty_ptr", 5u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_32(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_rec(v0) {
                        s.emit_enum_variant("syntax::ast::ty_rec", 6u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_34(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_fn(v0, v1) {
                        s.emit_enum_variant("syntax::ast::ty_fn", 7u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_37(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_tup(v0) {
                        s.emit_enum_variant("syntax::ast::ty_tup", 8u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_53(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_path(v0, v1) {
                        s.emit_enum_variant("syntax::ast::ty_path", 9u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_49(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_constr(v0, v1) {
                        s.emit_enum_variant("syntax::ast::ty_constr", 10u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_59(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_mac(v0) {
                        s.emit_enum_variant("syntax::ast::ty_mac", 11u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_67(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_infer {
                        s.emit_enum_variant("syntax::ast::ty_infer", 12u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*syntax::ast::ty*/
fn serialize_30<S: std::serialization::serializer>(s: S, v: syntax::ast::ty) {

    s.emit_rec(/*syntax::ast::ty_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_31(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::ty*/
fn serialize_29<S: std::serialization::serializer>(s: S,
                                                   v: @syntax::ast::ty) {

    s.emit_box(/*syntax::ast::ty*/{|| serialize_30(s, *v) });
}
/*syntax::ast::ty_param_bound*/
fn serialize_135<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::ty_param_bound) {
    s.emit_enum("syntax::ast::ty_param_bound",


                /*@syntax::ast::ty*/
                {||
                    alt v {
                      syntax::ast::bound_copy {
                        s.emit_enum_variant("syntax::ast::bound_copy", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::bound_send {
                        s.emit_enum_variant("syntax::ast::bound_send", 1u, 0u,
                                            {|| })
                      }
                      syntax::ast::bound_iface(v0) {
                        s.emit_enum_variant("syntax::ast::bound_iface", 2u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*[syntax::ast::ty_param_bound]*/
fn serialize_134<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::ty_param_bound]) {
    s.emit_vec(vec::len(v), /*syntax::ast::ty_param_bound*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_135(s, e) })
                              })
               });
}
/*@[syntax::ast::ty_param_bound]*/
fn serialize_133<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        @[syntax::ast::ty_param_bound]) {
    s.emit_box(/*[syntax::ast::ty_param_bound]*/{|| serialize_134(s, *v) });
}
/*syntax::ast::ty_param*/
fn serialize_132<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::ty_param) {
    s.emit_rec(/*syntax::ast::ident*//*syntax::ast::node_id*/
               /*@[syntax::ast::ty_param_bound]*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("id", 1u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("bounds", 2u,
                                        {|| serialize_133(s, v.bounds) })
                   }
               });
}
/*[syntax::ast::ty_param]*/
fn serialize_131<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::ty_param]) {
    s.emit_vec(vec::len(v), /*syntax::ast::ty_param*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_132(s, e) })
                              })
               });
}
/*[@syntax::ast::item]*/
fn serialize_137<S: std::serialization::serializer>(s: S,
                                                    v: [@syntax::ast::item]) {

    s.emit_vec(vec::len(v), /*@syntax::ast::item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_117(s, e) })
                              })
               });
}
/*syntax::ast::_mod*/
fn serialize_136<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::_mod) {

    s.emit_rec(/*[@syntax::ast::view_item]*//*[@syntax::ast::item]*/
               {||
                   {
                       s.emit_rec_field("view_items", 0u,
                                        {|| serialize_83(s, v.view_items) });
                       s.emit_rec_field("items", 1u,
                                        {|| serialize_137(s, v.items) })
                   }
               });
}
/*syntax::ast::native_item_*/
fn serialize_142<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::native_item_) {
    s.emit_enum("syntax::ast::native_item_",
                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                {||
                    alt v {
                      syntax::ast::native_item_fn(v0, v1) {
                        s.emit_enum_variant("syntax::ast::native_item_fn", 0u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::native_item*/
fn serialize_141<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::native_item) {
    s.emit_rec(/*syntax::ast::ident*//*[syntax::ast::attribute]*/
               /*syntax::ast::native_item_*//*syntax::ast::node_id*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("attrs", 1u,
                                        {|| serialize_2(s, v.attrs) });
                       s.emit_rec_field("node", 2u,
                                        {|| serialize_142(s, v.node) });
                       s.emit_rec_field("id", 3u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("span", 4u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::native_item*/
fn serialize_140<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        @syntax::ast::native_item) {
    s.emit_box(/*syntax::ast::native_item*/{|| serialize_141(s, *v) });
}
/*[@syntax::ast::native_item]*/
fn serialize_139<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [@syntax::ast::native_item]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::native_item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_140(s, e) })
                              })
               });
}
/*syntax::ast::native_mod*/
fn serialize_138<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::native_mod) {
    s.emit_rec(/*[@syntax::ast::view_item]*//*[@syntax::ast::native_item]*/
               {||
                   {
                       s.emit_rec_field("view_items", 0u,
                                        {|| serialize_83(s, v.view_items) });
                       s.emit_rec_field("items", 1u,
                                        {|| serialize_139(s, v.items) })
                   }
               });
}
/*syntax::ast::variant_arg*/
fn serialize_147<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::variant_arg) {
    s.emit_rec(/*@syntax::ast::ty*//*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("ty", 0u,
                                        {|| serialize_29(s, v.ty) });
                       s.emit_rec_field("id", 1u, {|| serialize_27(s, v.id) })
                   }
               });
}
/*[syntax::ast::variant_arg]*/
fn serialize_146<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::variant_arg]) {
    s.emit_vec(vec::len(v), /*syntax::ast::variant_arg*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_147(s, e) })
                              })
               });
}
/*syntax::ast::variant_*/
fn serialize_145<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::variant_) {
    s.emit_rec(/*syntax::ast::ident*//*[syntax::ast::attribute]*/
               /*[syntax::ast::variant_arg]*//*syntax::ast::node_id*/
               /*core::option::t<@syntax::ast::expr>*/
               {||
                   {
                       s.emit_rec_field("name", 0u,
                                        {|| serialize_1(s, v.name) });
                       s.emit_rec_field("attrs", 1u,
                                        {|| serialize_2(s, v.attrs) });
                       s.emit_rec_field("args", 2u,
                                        {|| serialize_146(s, v.args) });
                       s.emit_rec_field("id", 3u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("disr_expr", 4u,
                                        {|| serialize_77(s, v.disr_expr) })
                   }
               });
}
/*syntax::ast::variant*/
fn serialize_144<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::variant) {

    s.emit_rec(/*syntax::ast::variant_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_145(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::variant]*/
fn serialize_143<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::variant]) {
    s.emit_vec(vec::len(v), /*syntax::ast::variant*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_144(s, e) })
                              })
               });
}
/*syntax::ast::privacy*/
fn serialize_152<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::privacy) {

    s.emit_enum("syntax::ast::privacy",

                {||
                    alt v {
                      syntax::ast::priv {
                        s.emit_enum_variant("syntax::ast::priv", 0u, 0u,
                                            {|| })
                      }
                      syntax::ast::pub {
                        s.emit_enum_variant("syntax::ast::pub", 1u, 0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::class_mutability*/
fn serialize_154<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_mutability) {
    s.emit_enum("syntax::ast::class_mutability",

                {||
                    alt v {
                      syntax::ast::class_mutable {
                        s.emit_enum_variant("syntax::ast::class_mutable", 0u,
                                            0u, {|| })
                      }
                      syntax::ast::class_immutable {
                        s.emit_enum_variant("syntax::ast::class_immutable",
                                            1u, 0u, {|| })
                      }
                    }
                });
}
/*syntax::ast::class_member*/
fn serialize_153<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_member) {
    s.emit_enum("syntax::ast::class_member",
                /*syntax::ast::ident*//*@syntax::ast::ty*/
                /*syntax::ast::class_mutability*//*syntax::ast::node_id*/
                /*@syntax::ast::item*/
                {||
                    alt v {
                      syntax::ast::instance_var(v0, v1, v2, v3) {
                        s.emit_enum_variant("syntax::ast::instance_var", 0u,
                                            4u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_1(s,
                                                                                            v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_154(s,
                                                                                              v2)
                                                                            });
                                                    s.emit_enum_variant_arg(3u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v3)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::class_method(v0) {
                        s.emit_enum_variant("syntax::ast::class_method", 1u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_117(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::class_item_*/
fn serialize_151<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_item_) {
    s.emit_rec(/*syntax::ast::privacy*//*syntax::ast::class_member*/
               {||
                   {
                       s.emit_rec_field("privacy", 0u,
                                        {|| serialize_152(s, v.privacy) });
                       s.emit_rec_field("decl", 1u,
                                        {|| serialize_153(s, v.decl) })
                   }
               });
}
/*syntax::ast::class_item*/
fn serialize_150<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_item) {
    s.emit_rec(/*syntax::ast::class_item_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_151(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::class_item*/
fn serialize_149<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        @syntax::ast::class_item) {
    s.emit_box(/*syntax::ast::class_item*/{|| serialize_150(s, *v) });
}
/*[@syntax::ast::class_item]*/
fn serialize_148<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [@syntax::ast::class_item]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::class_item*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_149(s, e) })
                              })
               });
}
/*syntax::ast::class_ctor_*/
fn serialize_156<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_ctor_) {
    s.emit_rec(/*syntax::ast::node_id*//*syntax::ast::fn_decl*/
               /*syntax::ast::blk*/
               {||
                   {
                       s.emit_rec_field("id", 0u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("dec", 1u,
                                        {|| serialize_38(s, v.dec) });
                       s.emit_rec_field("body", 2u,
                                        {|| serialize_81(s, v.body) })
                   }
               });
}
/*syntax::ast::class_ctor*/
fn serialize_155<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::class_ctor) {
    s.emit_rec(/*syntax::ast::class_ctor_*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("node", 0u,
                                        {|| serialize_156(s, v.node) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*syntax::ast::ty_method*/
fn serialize_158<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::ty_method) {
    s.emit_rec(/*syntax::ast::ident*//*[syntax::ast::attribute]*/
               /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("attrs", 1u,
                                        {|| serialize_2(s, v.attrs) });
                       s.emit_rec_field("decl", 2u,
                                        {|| serialize_38(s, v.decl) });
                       s.emit_rec_field("tps", 3u,
                                        {|| serialize_131(s, v.tps) });
                       s.emit_rec_field("span", 4u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*[syntax::ast::ty_method]*/
fn serialize_157<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [syntax::ast::ty_method]) {
    s.emit_vec(vec::len(v), /*syntax::ast::ty_method*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_158(s, e) })
                              })
               });
}
/*core::option::t<@syntax::ast::ty>*/
fn serialize_159<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        core::option::t<@syntax::ast::ty>) {
    s.emit_enum("core::option::t",

                /*@syntax::ast::ty*/
                {||
                    alt v {
                      core::option::none {
                        s.emit_enum_variant("core::option::none", 0u, 0u,
                                            {|| })
                      }
                      core::option::some(v0) {
                        s.emit_enum_variant("core::option::some", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::method*/
fn serialize_162<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::method) {

    s.emit_rec(/*syntax::ast::ident*//*[syntax::ast::attribute]*/
               /*[syntax::ast::ty_param]*//*syntax::ast::fn_decl*/
               /*syntax::ast::blk*//*syntax::ast::node_id*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("attrs", 1u,
                                        {|| serialize_2(s, v.attrs) });
                       s.emit_rec_field("tps", 2u,
                                        {|| serialize_131(s, v.tps) });
                       s.emit_rec_field("decl", 3u,
                                        {|| serialize_38(s, v.decl) });
                       s.emit_rec_field("body", 4u,
                                        {|| serialize_81(s, v.body) });
                       s.emit_rec_field("id", 5u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("span", 6u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
/*@syntax::ast::method*/
fn serialize_161<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::method) {

    s.emit_box(/*syntax::ast::method*/{|| serialize_162(s, *v) });
}
/*[@syntax::ast::method]*/
fn serialize_160<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        [@syntax::ast::method]) {
    s.emit_vec(vec::len(v), /*@syntax::ast::method*/
               {||
                   vec::iteri(v,
                              {|i, e|
                                  s.emit_vec_elt(i, {|| serialize_161(s, e) })
                              })
               });
}
/*syntax::ast::item_*/
fn serialize_28<S: std::serialization::serializer>(s: S,
                                                   v: syntax::ast::item_) {

    s.emit_enum("syntax::ast::item_",
                /*@syntax::ast::ty*//*@syntax::ast::expr*/
                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                /*syntax::ast::blk*/
                /*syntax::ast::_mod*/
                /*syntax::ast::native_mod*/
                /*@syntax::ast::ty*//*[syntax::ast::ty_param]*/
                /*[syntax::ast::variant]*//*[syntax::ast::ty_param]*/
                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                /*syntax::ast::blk*//*syntax::ast::node_id*/
                /*syntax::ast::node_id*/
                /*[syntax::ast::ty_param]*//*[@syntax::ast::class_item]*/
                /*syntax::ast::class_ctor*/
                /*[syntax::ast::ty_param]*//*[syntax::ast::ty_method]*/
                /*[syntax::ast::ty_param]*/
                /*core::option::t<@syntax::ast::ty>*//*@syntax::ast::ty*/
                /*[@syntax::ast::method]*/
                {||
                    alt v {
                      syntax::ast::item_const(v0, v1) {
                        s.emit_enum_variant("syntax::ast::item_const", 0u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_70(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_fn(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::item_fn", 1u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_mod(v0) {
                        s.emit_enum_variant("syntax::ast::item_mod", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_136(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_native_mod(v0) {
                        s.emit_enum_variant("syntax::ast::item_native_mod",
                                            3u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_138(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_ty(v0, v1) {
                        s.emit_enum_variant("syntax::ast::item_ty", 4u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_enum(v0, v1) {
                        s.emit_enum_variant("syntax::ast::item_enum", 5u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_143(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_res(v0, v1, v2, v3, v4) {
                        s.emit_enum_variant("syntax::ast::item_res", 6u, 5u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_38(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_81(s,
                                                                                             v2)
                                                                            });
                                                    s.emit_enum_variant_arg(3u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v3)
                                                                            });
                                                    s.emit_enum_variant_arg(4u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v4)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_class(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::item_class", 7u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_148(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_155(s,
                                                                                              v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_iface(v0, v1) {
                        s.emit_enum_variant("syntax::ast::item_iface", 8u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_157(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::item_impl(v0, v1, v2, v3) {
                        s.emit_enum_variant("syntax::ast::item_impl", 9u, 4u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_131(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_159(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_29(s,
                                                                                             v2)
                                                                            });
                                                    s.emit_enum_variant_arg(3u,
                                                                            {||
                                                                                serialize_160(s,
                                                                                              v3)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
/*syntax::ast::item*/
fn serialize_0<S: std::serialization::serializer>(s: S,
                                                  v: syntax::ast::item) {

    s.emit_rec(/*syntax::ast::ident*//*[syntax::ast::attribute]*/
               /*syntax::ast::node_id*//*syntax::ast::item_*/
               /*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("ident", 0u,
                                        {|| serialize_1(s, v.ident) });
                       s.emit_rec_field("attrs", 1u,
                                        {|| serialize_2(s, v.attrs) });
                       s.emit_rec_field("id", 2u,
                                        {|| serialize_27(s, v.id) });
                       s.emit_rec_field("node", 3u,
                                        {|| serialize_28(s, v.node) });
                       s.emit_rec_field("span", 4u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
fn serialize_syntax_ast_item<S: std::serialization::serializer>(s: S,
                                                                v:
                                                                    syntax::ast::item) {
    serialize_0(s, v);
}
/*syntax::ast::ident*/
fn deserialize_1<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ident {
    s.read_str()
}
/*syntax::ast::attr_style*/
fn deserialize_5<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::attr_style {
    s.read_enum("syntax::ast::attr_style",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::attr_outer }
                                              1u { syntax::ast::attr_inner }
                                            }
                                        })
                })
}
/*@syntax::ast::meta_item*/
fn deserialize_9<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::meta_item {

    s.read_box(/*syntax::ast::meta_item*/{|| @deserialize_6(s) })

}
/*[@syntax::ast::meta_item]*/
fn deserialize_8<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::meta_item] {
    s.read_vec(

               /*@syntax::ast::meta_item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_9(s) })
                                })
               })
}
/*str*/
fn deserialize_12<S: std::serialization::deserializer>(s: S) -> str {
    s.read_str()
}
/*i64*/
fn deserialize_13<S: std::serialization::deserializer>(s: S) -> i64 {
    s.read_i64()
}
/*syntax::ast::int_ty*/
fn deserialize_14<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::int_ty {
    s.read_enum("syntax::ast::int_ty",











                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::ty_i }
                                              1u { syntax::ast::ty_char }
                                              2u { syntax::ast::ty_i8 }
                                              3u { syntax::ast::ty_i16 }
                                              4u { syntax::ast::ty_i32 }
                                              5u { syntax::ast::ty_i64 }
                                            }
                                        })
                })
}
/*u64*/
fn deserialize_15<S: std::serialization::deserializer>(s: S) -> u64 {
    s.read_u64()
}
/*syntax::ast::uint_ty*/
fn deserialize_16<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::uint_ty {
    s.read_enum("syntax::ast::uint_ty",









                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::ty_u }
                                              1u { syntax::ast::ty_u8 }
                                              2u { syntax::ast::ty_u16 }
                                              3u { syntax::ast::ty_u32 }
                                              4u { syntax::ast::ty_u64 }
                                            }
                                        })
                })
}
/*syntax::ast::float_ty*/
fn deserialize_17<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::float_ty {
    s.read_enum("syntax::ast::float_ty",





                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::ty_f }
                                              1u { syntax::ast::ty_f32 }
                                              2u { syntax::ast::ty_f64 }
                                            }
                                        })
                })
}
/*bool*/
fn deserialize_18<S: std::serialization::deserializer>(s: S) -> bool {
    s.read_bool()
}
/*syntax::ast::lit_*/
fn deserialize_11<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::lit_ {
    s.read_enum("syntax::ast::lit_",
                /*str*/

                /*i64*//*syntax::ast::int_ty*/

                /*u64*//*syntax::ast::uint_ty*/

                /*str*//*syntax::ast::float_ty*/



                /*bool*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::lit_str(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_12(s)
                                                                                             }))
                                              }
                                              1u {
                                                syntax::ast::lit_int(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_13(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_14(s)
                                                                                             }))
                                              }
                                              2u {
                                                syntax::ast::lit_uint(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_15(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_16(s)
                                                                                              }))
                                              }
                                              3u {
                                                syntax::ast::lit_float(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_12(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_17(s)
                                                                                               }))
                                              }
                                              4u { syntax::ast::lit_nil }
                                              5u {
                                                syntax::ast::lit_bool(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_18(s)
                                                                                              }))
                                              }
                                            }
                                        })
                })
}
/*uint*/
fn deserialize_20<S: std::serialization::deserializer>(s: S) -> uint {
    s.read_uint()
}
/*core::option::t<syntax::codemap::span>*/
fn deserialize_26<S: std::serialization::deserializer>(s: S) ->
   core::option::t<syntax::codemap::span> {
    s.read_enum("core::option::t",


                /*syntax::codemap::span*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_19(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*{name: str,span: core::option::t<syntax::codemap::span>}*/
fn deserialize_25<S: std::serialization::deserializer>(s: S) ->
   {name: str, span: core::option::t<syntax::codemap::span>,} {

    s.read_rec(


               /*str*/

               /*core::option::t<syntax::codemap::span>*/

               {||
                   {name:
                        s.read_rec_field("name", 0u, {|| deserialize_12(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_26(s) }),}
               })

}
/*{call_site: syntax::codemap::span,callie: {name: str,span: core::option::t<syntax::codemap::span>}}*/
fn deserialize_24<S: std::serialization::deserializer>(s: S) ->
   {call_site: syntax::codemap::span,
    callie: {name: str, span: core::option::t<syntax::codemap::span>,},} {

    s.read_rec(


               /*syntax::codemap::span*/

               /*{name: str,span: core::option::t<syntax::codemap::span>}*/

               {||
                   {call_site:
                        s.read_rec_field("call_site", 0u,
                                         {|| deserialize_19(s) }),
                    callie:
                        s.read_rec_field("callie", 1u,
                                         {|| deserialize_25(s) }),}
               })

}
/*syntax::codemap::expn_info_*/
fn deserialize_23<S: std::serialization::deserializer>(s: S) ->
   syntax::codemap::expn_info_ {
    s.read_enum("syntax::codemap::expn_info_",

                /*{call_site: syntax::codemap::span,callie: {name: str,span: core::option::t<syntax::codemap::span>}}*/

                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::codemap::expanded_from(s.read_enum_variant_arg(0u,
                                                                                                       {||
                                                                                                           deserialize_24(s)
                                                                                                       }))
                                              }
                                            }
                                        })
                })
}
/*@syntax::codemap::expn_info_*/
fn deserialize_22<S: std::serialization::deserializer>(s: S) ->
   @syntax::codemap::expn_info_ {

    s.read_box(/*syntax::codemap::expn_info_*/{|| @deserialize_23(s) })

}
/*syntax::codemap::expn_info<@syntax::codemap::expn_info_>*/
fn deserialize_21<S: std::serialization::deserializer>(s: S) ->
   syntax::codemap::expn_info<@syntax::codemap::expn_info_> {
    s.read_enum("core::option::t",


                /*@syntax::codemap::expn_info_*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_22(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::codemap::span*/
fn deserialize_19<S: std::serialization::deserializer>(s: S) ->
   syntax::codemap::span {

    s.read_rec(


               /*uint*/

               /*uint*/

               /*syntax::codemap::expn_info<@syntax::codemap::expn_info_>*/

               {||
                   {lo: s.read_rec_field("lo", 0u, {|| deserialize_20(s) }),
                    hi: s.read_rec_field("hi", 1u, {|| deserialize_20(s) }),
                    expn_info:
                        s.read_rec_field("expn_info", 2u,
                                         {|| deserialize_21(s) }),}
               })

}
/*syntax::ast::lit*/
fn deserialize_10<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::lit {

    s.read_rec(


               /*syntax::ast::lit_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_11(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::meta_item_*/
fn deserialize_7<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::meta_item_ {
    s.read_enum("syntax::ast::meta_item_",
                /*syntax::ast::ident*/

                /*syntax::ast::ident*//*[@syntax::ast::meta_item]*/

                /*syntax::ast::ident*//*syntax::ast::lit*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::meta_word(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_1(s)
                                                                                               }))
                                              }
                                              1u {
                                                syntax::ast::meta_list(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_1(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_8(s)
                                                                                               }))
                                              }
                                              2u {
                                                syntax::ast::meta_name_value(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_1(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(1u,
                                                                                                     {||
                                                                                                         deserialize_10(s)
                                                                                                     }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::meta_item*/
fn deserialize_6<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::meta_item {

    s.read_rec(


               /*syntax::ast::meta_item_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_7(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::attribute_*/
fn deserialize_4<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::attribute_ {

    s.read_rec(


               /*syntax::ast::attr_style*/

               /*syntax::ast::meta_item*/

               {||
                   {style:
                        s.read_rec_field("style", 0u, {|| deserialize_5(s) }),
                    value:
                        s.read_rec_field("value", 1u,
                                         {|| deserialize_6(s) }),}
               })

}
/*syntax::ast::attribute*/
fn deserialize_3<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::attribute {

    s.read_rec(


               /*syntax::ast::attribute_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_4(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::attribute]*/
fn deserialize_2<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::attribute] {
    s.read_vec(

               /*syntax::ast::attribute*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_3(s) })
                                })
               })
}
/*syntax::ast::node_id*/
fn deserialize_27<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::node_id {
    s.read_int()
}
/*syntax::ast::mutability*/
fn deserialize_33<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mutability {
    s.read_enum("syntax::ast::mutability",





                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::m_mutbl }
                                              1u { syntax::ast::m_imm }
                                              2u { syntax::ast::m_const }
                                            }
                                        })
                })
}
/*syntax::ast::mt*/
fn deserialize_32<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mt {

    s.read_rec(


               /*@syntax::ast::ty*/

               /*syntax::ast::mutability*/

               {||
                   {ty: s.read_rec_field("ty", 0u, {|| deserialize_29(s) }),
                    mutbl:
                        s.read_rec_field("mutbl", 1u,
                                         {|| deserialize_33(s) }),}
               })

}
/*syntax::ast::ty_field_*/
fn deserialize_36<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_field_ {

    s.read_rec(


               /*syntax::ast::ident*/

               /*syntax::ast::mt*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    mt: s.read_rec_field("mt", 1u, {|| deserialize_32(s) }),}
               })
}
/*syntax::ast::ty_field*/
fn deserialize_35<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_field {

    s.read_rec(


               /*syntax::ast::ty_field_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_36(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::ty_field]*/
fn deserialize_34<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::ty_field] {
    s.read_vec(

               /*syntax::ast::ty_field*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_35(s) })
                                })
               })
}
/*syntax::ast::proto*/
fn deserialize_37<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::proto {
    s.read_enum("syntax::ast::proto",









                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::proto_bare }
                                              1u { syntax::ast::proto_any }
                                              2u { syntax::ast::proto_uniq }
                                              3u { syntax::ast::proto_box }
                                              4u { syntax::ast::proto_block }
                                            }
                                        })
                })
}
/*syntax::ast::rmode*/
fn deserialize_42<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::rmode {
    s.read_enum("syntax::ast::rmode",









                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::by_ref }
                                              1u { syntax::ast::by_val }
                                              2u { syntax::ast::by_mutbl_ref }
                                              3u { syntax::ast::by_move }
                                              4u { syntax::ast::by_copy }
                                            }
                                        })
                })
}
/*syntax::ast::mode<syntax::ast::rmode>*/
fn deserialize_41<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mode<syntax::ast::rmode> {
    s.read_enum("syntax::ast::inferable",
                /*syntax::ast::rmode*/

                /*syntax::ast::node_id*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::expl(s.read_enum_variant_arg(0u,
                                                                                          {||
                                                                                              deserialize_42(s)
                                                                                          }))
                                              }
                                              1u {
                                                syntax::ast::infer(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_27(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::arg*/
fn deserialize_40<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::arg {

    s.read_rec(


               /*syntax::ast::mode<syntax::ast::rmode>*/

               /*@syntax::ast::ty*/

               /*syntax::ast::ident*/

               /*syntax::ast::node_id*/

               {||
                   {mode:
                        s.read_rec_field("mode", 0u, {|| deserialize_41(s) }),
                    ty: s.read_rec_field("ty", 1u, {|| deserialize_29(s) }),
                    ident:
                        s.read_rec_field("ident", 2u, {|| deserialize_1(s) }),
                    id: s.read_rec_field("id", 3u, {|| deserialize_27(s) }),}
               })
}
/*[syntax::ast::arg]*/
fn deserialize_39<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::arg] {
    s.read_vec(

               /*syntax::ast::arg*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_40(s) })
                                })
               })
}
/*syntax::ast::purity*/
fn deserialize_43<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::purity {
    s.read_enum("syntax::ast::purity",







                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::pure_fn }
                                              1u { syntax::ast::unsafe_fn }
                                              2u { syntax::ast::impure_fn }
                                              3u { syntax::ast::crust_fn }
                                            }
                                        })
                })
}
/*syntax::ast::ret_style*/
fn deserialize_44<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ret_style {
    s.read_enum("syntax::ast::ret_style",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::noreturn }
                                              1u { syntax::ast::return_val }
                                            }
                                        })
                })
}
/*[syntax::ast::ident]*/
fn deserialize_52<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::ident] {
    s.read_vec(

               /*syntax::ast::ident*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_1(s) })
                                })
               })
}
/*[@syntax::ast::ty]*/
fn deserialize_53<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::ty] {
    s.read_vec(

               /*@syntax::ast::ty*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_29(s) })
                                })
               })
}
/*syntax::ast::path_*/
fn deserialize_51<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::path_ {

    s.read_rec(


               /*bool*/

               /*[syntax::ast::ident]*/

               /*[@syntax::ast::ty]*/

               {||
                   {global:
                        s.read_rec_field("global", 0u,
                                         {|| deserialize_18(s) }),
                    idents:
                        s.read_rec_field("idents", 1u,
                                         {|| deserialize_52(s) }),
                    types:
                        s.read_rec_field("types", 2u,
                                         {|| deserialize_53(s) }),}
               })

}
/*syntax::ast::path*/
fn deserialize_50<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::path {

    s.read_rec(


               /*syntax::ast::path_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_51(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::path*/
fn deserialize_49<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::path {

    s.read_box(/*syntax::ast::path*/{|| @deserialize_50(s) })

}
/*@syntax::ast::lit*/
fn deserialize_58<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::lit {

    s.read_box(/*syntax::ast::lit*/{|| @deserialize_10(s) })

}
/*syntax::ast::constr_arg_general_<uint>*/
fn deserialize_57<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::constr_arg_general_<uint> {
    s.read_enum("syntax::ast::constr_arg_general_",


                /*uint*/

                /*@syntax::ast::lit*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::carg_base }
                                              1u {
                                                syntax::ast::carg_ident(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_20(s)
                                                                                                }))
                                              }
                                              2u {
                                                syntax::ast::carg_lit(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_58(s)
                                                                                              }))
                                              }
                                            }
                                        })
                })
}
/*{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
fn deserialize_56<S: std::serialization::deserializer>(s: S) ->
   {node: syntax::ast::constr_arg_general_<uint>,
    span: syntax::codemap::span,} {

    s.read_rec(


               /*syntax::ast::constr_arg_general_<uint>*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_57(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
fn deserialize_55<S: std::serialization::deserializer>(s: S) ->
   @{node: syntax::ast::constr_arg_general_<uint>,
     span: syntax::codemap::span,} {

    s.read_box(
               /*{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/
               {|| @deserialize_56(s) })

}
/*[@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}]*/
fn deserialize_54<S: std::serialization::deserializer>(s: S) ->
   [@{node: syntax::ast::constr_arg_general_<uint>,
      span: syntax::codemap::span,}] {
    s.read_vec(


               /*@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}*/

               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_55(s) })
                                })
               })
}
/*{path: @syntax::ast::path,args: [@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}],id: syntax::ast::node_id}*/
fn deserialize_48<S: std::serialization::deserializer>(s: S) ->
   {path: @syntax::ast::path,
    args:
        [@{node: syntax::ast::constr_arg_general_<uint>,
           span: syntax::codemap::span,}],
    id: syntax::ast::node_id,} {

    s.read_rec(


               /*@syntax::ast::path*/


               /*[@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}]*/


               /*syntax::ast::node_id*/

               {||
                   {path:
                        s.read_rec_field("path", 0u, {|| deserialize_49(s) }),
                    args:
                        s.read_rec_field("args", 1u, {|| deserialize_54(s) }),
                    id: s.read_rec_field("id", 2u, {|| deserialize_27(s) }),}
               })
}
/*syntax::ast::constr*/
fn deserialize_47<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::constr {

    s.read_rec(



               /*{path: @syntax::ast::path,args: [@{node: syntax::ast::constr_arg_general_<uint>,span: syntax::codemap::span}],id: syntax::ast::node_id}*/


               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_48(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::constr*/
fn deserialize_46<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::constr {

    s.read_box(/*syntax::ast::constr*/{|| @deserialize_47(s) })

}
/*[@syntax::ast::constr]*/
fn deserialize_45<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::constr] {
    s.read_vec(

               /*@syntax::ast::constr*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_46(s) })
                                })
               })
}
/*syntax::ast::fn_decl*/
fn deserialize_38<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::fn_decl {

    s.read_rec(


               /*[syntax::ast::arg]*/

               /*@syntax::ast::ty*/

               /*syntax::ast::purity*/

               /*syntax::ast::ret_style*/

               /*[@syntax::ast::constr]*/

               {||
                   {inputs:
                        s.read_rec_field("inputs", 0u,
                                         {|| deserialize_39(s) }),
                    output:
                        s.read_rec_field("output", 1u,
                                         {|| deserialize_29(s) }),
                    purity:
                        s.read_rec_field("purity", 2u,
                                         {|| deserialize_43(s) }),
                    cf: s.read_rec_field("cf", 3u, {|| deserialize_44(s) }),
                    constraints:
                        s.read_rec_field("constraints", 4u,
                                         {|| deserialize_45(s) }),}
               })

}
/*syntax::ast::constr_arg_general_<@syntax::ast::path>*/
fn deserialize_66<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::constr_arg_general_<@syntax::ast::path> {
    s.read_enum("syntax::ast::constr_arg_general_",


                /*@syntax::ast::path*/

                /*@syntax::ast::lit*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::carg_base }
                                              1u {
                                                syntax::ast::carg_ident(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_49(s)
                                                                                                }))
                                              }
                                              2u {
                                                syntax::ast::carg_lit(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_58(s)
                                                                                              }))
                                              }
                                            }
                                        })
                })
}
/*{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
fn deserialize_65<S: std::serialization::deserializer>(s: S) ->
   {node: syntax::ast::constr_arg_general_<@syntax::ast::path>,
    span: syntax::codemap::span,} {

    s.read_rec(


               /*syntax::ast::constr_arg_general_<@syntax::ast::path>*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_66(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
fn deserialize_64<S: std::serialization::deserializer>(s: S) ->
   @{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,
     span: syntax::codemap::span,} {

    s.read_box(
               /*{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/
               {|| @deserialize_65(s) })

}
/*[@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}]*/
fn deserialize_63<S: std::serialization::deserializer>(s: S) ->
   [@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,
      span: syntax::codemap::span,}] {
    s.read_vec(


               /*@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}*/

               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_64(s) })
                                })
               })
}
/*syntax::ast::ty_constr_*/
fn deserialize_62<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_constr_ {

    s.read_rec(


               /*@syntax::ast::path*/


               /*[@{node: syntax::ast::constr_arg_general_<@syntax::ast::path>,span: syntax::codemap::span}]*/


               /*syntax::ast::node_id*/

               {||
                   {path:
                        s.read_rec_field("path", 0u, {|| deserialize_49(s) }),
                    args:
                        s.read_rec_field("args", 1u, {|| deserialize_63(s) }),
                    id: s.read_rec_field("id", 2u, {|| deserialize_27(s) }),}
               })
}
/*syntax::ast::ty_constr*/
fn deserialize_61<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_constr {

    s.read_rec(


               /*syntax::ast::ty_constr_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_62(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::ty_constr*/
fn deserialize_60<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::ty_constr {

    s.read_box(/*syntax::ast::ty_constr*/{|| @deserialize_61(s) })

}
/*[@syntax::ast::ty_constr]*/
fn deserialize_59<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::ty_constr] {
    s.read_vec(

               /*@syntax::ast::ty_constr*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_60(s) })
                                })
               })
}
/*[@syntax::ast::expr]*/
fn deserialize_73<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::expr] {
    s.read_vec(

               /*@syntax::ast::expr*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_70(s) })
                                })
               })
}
/*syntax::ast::field_*/
fn deserialize_76<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::field_ {

    s.read_rec(


               /*syntax::ast::mutability*/

               /*syntax::ast::ident*/

               /*@syntax::ast::expr*/

               {||
                   {mutbl:
                        s.read_rec_field("mutbl", 0u,
                                         {|| deserialize_33(s) }),
                    ident:
                        s.read_rec_field("ident", 1u, {|| deserialize_1(s) }),
                    expr:
                        s.read_rec_field("expr", 2u,
                                         {|| deserialize_70(s) }),}
               })

}
/*syntax::ast::field*/
fn deserialize_75<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::field {

    s.read_rec(


               /*syntax::ast::field_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_76(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::field]*/
fn deserialize_74<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::field] {
    s.read_vec(

               /*syntax::ast::field*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_75(s) })
                                })
               })
}
/*core::option::t<@syntax::ast::expr>*/
fn deserialize_77<S: std::serialization::deserializer>(s: S) ->
   core::option::t<@syntax::ast::expr> {
    s.read_enum("core::option::t",


                /*@syntax::ast::expr*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_70(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*[core::option::t<@syntax::ast::expr>]*/
fn deserialize_78<S: std::serialization::deserializer>(s: S) ->
   [core::option::t<@syntax::ast::expr>] {
    s.read_vec(

               /*core::option::t<@syntax::ast::expr>*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_77(s) })
                                })
               })
}
/*syntax::ast::binop*/
fn deserialize_79<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::binop {
    s.read_enum("syntax::ast::binop",





































                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::add }
                                              1u { syntax::ast::subtract }
                                              2u { syntax::ast::mul }
                                              3u { syntax::ast::div }
                                              4u { syntax::ast::rem }
                                              5u { syntax::ast::and }
                                              6u { syntax::ast::or }
                                              7u { syntax::ast::bitxor }
                                              8u { syntax::ast::bitand }
                                              9u { syntax::ast::bitor }
                                              10u { syntax::ast::lsl }
                                              11u { syntax::ast::lsr }
                                              12u { syntax::ast::asr }
                                              13u { syntax::ast::eq }
                                              14u { syntax::ast::lt }
                                              15u { syntax::ast::le }
                                              16u { syntax::ast::ne }
                                              17u { syntax::ast::ge }
                                              18u { syntax::ast::gt }
                                            }
                                        })
                })
}
/*syntax::ast::unop*/
fn deserialize_80<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::unop {
    s.read_enum("syntax::ast::unop",
                /*syntax::ast::mutability*/

                /*syntax::ast::mutability*/






                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::box(s.read_enum_variant_arg(0u,
                                                                                         {||
                                                                                             deserialize_33(s)
                                                                                         }))
                                              }
                                              1u {
                                                syntax::ast::uniq(s.read_enum_variant_arg(0u,
                                                                                          {||
                                                                                              deserialize_33(s)
                                                                                          }))
                                              }
                                              2u { syntax::ast::deref }
                                              3u { syntax::ast::not }
                                              4u { syntax::ast::neg }
                                            }
                                        })
                })
}
/*syntax::ast::simple_path*/
fn deserialize_92<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::simple_path {
    s.read_vec(

               /*syntax::ast::ident*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_1(s) })
                                })
               })
}
/*@syntax::ast::simple_path*/
fn deserialize_91<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::simple_path {

    s.read_box(/*syntax::ast::simple_path*/{|| @deserialize_92(s) })

}
/*syntax::ast::path_list_ident_*/
fn deserialize_95<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::path_list_ident_ {

    s.read_rec(


               /*syntax::ast::ident*/

               /*syntax::ast::node_id*/

               {||
                   {name:
                        s.read_rec_field("name", 0u, {|| deserialize_1(s) }),
                    id: s.read_rec_field("id", 1u, {|| deserialize_27(s) }),}
               })
}
/*syntax::ast::path_list_ident*/
fn deserialize_94<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::path_list_ident {

    s.read_rec(


               /*syntax::ast::path_list_ident_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_95(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::path_list_ident]*/
fn deserialize_93<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::path_list_ident] {
    s.read_vec(

               /*syntax::ast::path_list_ident*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_94(s) })
                                })
               })
}
/*syntax::ast::view_path_*/
fn deserialize_90<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::view_path_ {
    s.read_enum("syntax::ast::view_path_",
                /*syntax::ast::ident*//*@syntax::ast::simple_path*/
                /*syntax::ast::node_id*/

                /*@syntax::ast::simple_path*//*syntax::ast::node_id*/

                /*@syntax::ast::simple_path*/
                /*[syntax::ast::path_list_ident]*//*syntax::ast::node_id*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::view_path_simple(s.read_enum_variant_arg(0u,
                                                                                                      {||
                                                                                                          deserialize_1(s)
                                                                                                      }),
                                                                              s.read_enum_variant_arg(1u,
                                                                                                      {||
                                                                                                          deserialize_91(s)
                                                                                                      }),
                                                                              s.read_enum_variant_arg(2u,
                                                                                                      {||
                                                                                                          deserialize_27(s)
                                                                                                      }))
                                              }
                                              1u {
                                                syntax::ast::view_path_glob(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_91(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(1u,
                                                                                                    {||
                                                                                                        deserialize_27(s)
                                                                                                    }))
                                              }
                                              2u {
                                                syntax::ast::view_path_list(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_91(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(1u,
                                                                                                    {||
                                                                                                        deserialize_93(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(2u,
                                                                                                    {||
                                                                                                        deserialize_27(s)
                                                                                                    }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::view_path*/
fn deserialize_89<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::view_path {

    s.read_rec(


               /*syntax::ast::view_path_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_90(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::view_path*/
fn deserialize_88<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::view_path {

    s.read_box(/*syntax::ast::view_path*/{|| @deserialize_89(s) })

}
/*[@syntax::ast::view_path]*/
fn deserialize_87<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::view_path] {
    s.read_vec(

               /*@syntax::ast::view_path*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_88(s) })
                                })
               })
}
/*syntax::ast::view_item_*/
fn deserialize_86<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::view_item_ {
    s.read_enum("syntax::ast::view_item_",
                /*syntax::ast::ident*//*[@syntax::ast::meta_item]*/
                /*syntax::ast::node_id*/

                /*[@syntax::ast::view_path]*/

                /*[@syntax::ast::view_path]*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::view_item_use(s.read_enum_variant_arg(0u,
                                                                                                   {||
                                                                                                       deserialize_1(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(1u,
                                                                                                   {||
                                                                                                       deserialize_8(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(2u,
                                                                                                   {||
                                                                                                       deserialize_27(s)
                                                                                                   }))
                                              }
                                              1u {
                                                syntax::ast::view_item_import(s.read_enum_variant_arg(0u,
                                                                                                      {||
                                                                                                          deserialize_87(s)
                                                                                                      }))
                                              }
                                              2u {
                                                syntax::ast::view_item_export(s.read_enum_variant_arg(0u,
                                                                                                      {||
                                                                                                          deserialize_87(s)
                                                                                                      }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::view_item*/
fn deserialize_85<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::view_item {

    s.read_rec(


               /*syntax::ast::view_item_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_86(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::view_item*/
fn deserialize_84<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::view_item {

    s.read_box(/*syntax::ast::view_item*/{|| @deserialize_85(s) })

}
/*[@syntax::ast::view_item]*/
fn deserialize_83<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::view_item] {
    s.read_vec(

               /*@syntax::ast::view_item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_84(s) })
                                })
               })
}
/*core::option::t<@syntax::ast::pat>*/
fn deserialize_110<S: std::serialization::deserializer>(s: S) ->
   core::option::t<@syntax::ast::pat> {
    s.read_enum("core::option::t",


                /*@syntax::ast::pat*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_107(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*[@syntax::ast::pat]*/
fn deserialize_111<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::pat] {
    s.read_vec(

               /*@syntax::ast::pat*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_107(s) })
                                })
               })
}
/*syntax::ast::field_pat*/
fn deserialize_113<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::field_pat {

    s.read_rec(


               /*syntax::ast::ident*/

               /*@syntax::ast::pat*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    pat:
                        s.read_rec_field("pat", 1u,
                                         {|| deserialize_107(s) }),}
               })

}
/*[syntax::ast::field_pat]*/
fn deserialize_112<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::field_pat] {
    s.read_vec(

               /*syntax::ast::field_pat*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_113(s) })
                                })
               })
}
/*syntax::ast::pat_*/
fn deserialize_109<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::pat_ {
    s.read_enum("syntax::ast::pat_",


                /*@syntax::ast::path*//*core::option::t<@syntax::ast::pat>*/

                /*@syntax::ast::path*//*[@syntax::ast::pat]*/

                /*[syntax::ast::field_pat]*//*bool*/

                /*[@syntax::ast::pat]*/

                /*@syntax::ast::pat*/

                /*@syntax::ast::pat*/

                /*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*@syntax::ast::expr*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::pat_wild }
                                              1u {
                                                syntax::ast::pat_ident(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_49(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_110(s)
                                                                                               }))
                                              }
                                              2u {
                                                syntax::ast::pat_enum(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_49(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_111(s)
                                                                                              }))
                                              }
                                              3u {
                                                syntax::ast::pat_rec(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_112(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_18(s)
                                                                                             }))
                                              }
                                              4u {
                                                syntax::ast::pat_tup(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_111(s)
                                                                                             }))
                                              }
                                              5u {
                                                syntax::ast::pat_box(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_107(s)
                                                                                             }))
                                              }
                                              6u {
                                                syntax::ast::pat_uniq(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_107(s)
                                                                                              }))
                                              }
                                              7u {
                                                syntax::ast::pat_lit(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_70(s)
                                                                                             }))
                                              }
                                              8u {
                                                syntax::ast::pat_range(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::pat*/
fn deserialize_108<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::pat {

    s.read_rec(


               /*syntax::ast::node_id*/

               /*syntax::ast::pat_*/

               /*syntax::codemap::span*/

               {||
                   {id: s.read_rec_field("id", 0u, {|| deserialize_27(s) }),
                    node:
                        s.read_rec_field("node", 1u,
                                         {|| deserialize_109(s) }),
                    span:
                        s.read_rec_field("span", 2u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::pat*/
fn deserialize_107<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::pat {

    s.read_box(/*syntax::ast::pat*/{|| @deserialize_108(s) })

}
/*syntax::ast::init_op*/
fn deserialize_116<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::init_op {
    s.read_enum("syntax::ast::init_op",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::init_assign }
                                              1u { syntax::ast::init_move }
                                            }
                                        })
                })
}
/*syntax::ast::initializer*/
fn deserialize_115<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::initializer {

    s.read_rec(


               /*syntax::ast::init_op*/

               /*@syntax::ast::expr*/

               {||
                   {op: s.read_rec_field("op", 0u, {|| deserialize_116(s) }),
                    expr:
                        s.read_rec_field("expr", 1u,
                                         {|| deserialize_70(s) }),}
               })

}
/*core::option::t<syntax::ast::initializer>*/
fn deserialize_114<S: std::serialization::deserializer>(s: S) ->
   core::option::t<syntax::ast::initializer> {
    s.read_enum("core::option::t",


                /*syntax::ast::initializer*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_115(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::local_*/
fn deserialize_106<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::local_ {

    s.read_rec(


               /*bool*/

               /*@syntax::ast::ty*/

               /*@syntax::ast::pat*/

               /*core::option::t<syntax::ast::initializer>*/

               /*syntax::ast::node_id*/

               {||
                   {is_mutbl:
                        s.read_rec_field("is_mutbl", 0u,
                                         {|| deserialize_18(s) }),
                    ty: s.read_rec_field("ty", 1u, {|| deserialize_29(s) }),
                    pat:
                        s.read_rec_field("pat", 2u, {|| deserialize_107(s) }),
                    init:
                        s.read_rec_field("init", 3u,
                                         {|| deserialize_114(s) }),
                    id: s.read_rec_field("id", 4u, {|| deserialize_27(s) }),}
               })
}
/*syntax::ast::local*/
fn deserialize_105<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::local {

    s.read_rec(


               /*syntax::ast::local_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u,
                                         {|| deserialize_106(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::local*/
fn deserialize_104<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::local {

    s.read_box(/*syntax::ast::local*/{|| @deserialize_105(s) })

}
/*[@syntax::ast::local]*/
fn deserialize_103<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::local] {
    s.read_vec(

               /*@syntax::ast::local*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_104(s) })
                                })
               })
}
/*@syntax::ast::item*/
fn deserialize_117<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::item {

    s.read_box(/*syntax::ast::item*/{|| @deserialize_0(s) })

}
/*syntax::ast::decl_*/
fn deserialize_102<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::decl_ {
    s.read_enum("syntax::ast::decl_",
                /*[@syntax::ast::local]*/

                /*@syntax::ast::item*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::decl_local(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_103(s)
                                                                                                }))
                                              }
                                              1u {
                                                syntax::ast::decl_item(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_117(s)
                                                                                               }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::decl*/
fn deserialize_101<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::decl {

    s.read_rec(


               /*syntax::ast::decl_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u,
                                         {|| deserialize_102(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::decl*/
fn deserialize_100<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::decl {

    s.read_box(/*syntax::ast::decl*/{|| @deserialize_101(s) })

}
/*syntax::ast::stmt_*/
fn deserialize_99<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::stmt_ {
    s.read_enum("syntax::ast::stmt_",
                /*@syntax::ast::decl*//*syntax::ast::node_id*/

                /*@syntax::ast::expr*//*syntax::ast::node_id*/

                /*@syntax::ast::expr*//*syntax::ast::node_id*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::stmt_decl(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_100(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }))
                                              }
                                              1u {
                                                syntax::ast::stmt_expr(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }))
                                              }
                                              2u {
                                                syntax::ast::stmt_semi(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::stmt*/
fn deserialize_98<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::stmt {

    s.read_rec(


               /*syntax::ast::stmt_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_99(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::stmt*/
fn deserialize_97<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::stmt {

    s.read_box(/*syntax::ast::stmt*/{|| @deserialize_98(s) })

}
/*[@syntax::ast::stmt]*/
fn deserialize_96<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::stmt] {
    s.read_vec(

               /*@syntax::ast::stmt*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i, {|| deserialize_97(s) })
                                })
               })
}
/*syntax::ast::blk_check_mode*/
fn deserialize_118<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::blk_check_mode {
    s.read_enum("syntax::ast::blk_check_mode",





                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::default_blk }
                                              1u {
                                                syntax::ast::unchecked_blk
                                              }
                                              2u { syntax::ast::unsafe_blk }
                                            }
                                        })
                })
}
/*syntax::ast::blk_*/
fn deserialize_82<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::blk_ {

    s.read_rec(


               /*[@syntax::ast::view_item]*/

               /*[@syntax::ast::stmt]*/

               /*core::option::t<@syntax::ast::expr>*/

               /*syntax::ast::node_id*/

               /*syntax::ast::blk_check_mode*/

               {||
                   {view_items:
                        s.read_rec_field("view_items", 0u,
                                         {|| deserialize_83(s) }),
                    stmts:
                        s.read_rec_field("stmts", 1u,
                                         {|| deserialize_96(s) }),
                    expr:
                        s.read_rec_field("expr", 2u, {|| deserialize_77(s) }),
                    id: s.read_rec_field("id", 3u, {|| deserialize_27(s) }),
                    rules:
                        s.read_rec_field("rules", 4u,
                                         {|| deserialize_118(s) }),}
               })

}
/*syntax::ast::blk*/
fn deserialize_81<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::blk {

    s.read_rec(


               /*syntax::ast::blk_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_82(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::arm*/
fn deserialize_120<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::arm {

    s.read_rec(


               /*[@syntax::ast::pat]*/

               /*core::option::t<@syntax::ast::expr>*/

               /*syntax::ast::blk*/

               {||
                   {pats:
                        s.read_rec_field("pats", 0u,
                                         {|| deserialize_111(s) }),
                    guard:
                        s.read_rec_field("guard", 1u,
                                         {|| deserialize_77(s) }),
                    body:
                        s.read_rec_field("body", 2u,
                                         {|| deserialize_81(s) }),}
               })

}
/*[syntax::ast::arm]*/
fn deserialize_119<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::arm] {
    s.read_vec(

               /*syntax::ast::arm*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_120(s) })
                                })
               })
}
/*syntax::ast::alt_mode*/
fn deserialize_121<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::alt_mode {
    s.read_enum("syntax::ast::alt_mode",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::alt_check }
                                              1u {
                                                syntax::ast::alt_exhaustive
                                              }
                                            }
                                        })
                })
}
/*int*/
fn deserialize_127<S: std::serialization::deserializer>(s: S) -> int {
    s.read_int()
}
/*syntax::ast::capture_item*/
fn deserialize_126<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::capture_item {

    s.read_rec(


               /*int*/

               /*syntax::ast::ident*/

               /*syntax::codemap::span*/

               {||
                   {id: s.read_rec_field("id", 0u, {|| deserialize_127(s) }),
                    name:
                        s.read_rec_field("name", 1u, {|| deserialize_1(s) }),
                    span:
                        s.read_rec_field("span", 2u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::capture_item*/
fn deserialize_125<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::capture_item {

    s.read_box(/*syntax::ast::capture_item*/{|| @deserialize_126(s) })

}
/*[@syntax::ast::capture_item]*/
fn deserialize_124<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::capture_item] {
    s.read_vec(

               /*@syntax::ast::capture_item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_125(s) })
                                })
               })
}
/*syntax::ast::capture_clause*/
fn deserialize_123<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::capture_clause {

    s.read_rec(


               /*[@syntax::ast::capture_item]*/

               /*[@syntax::ast::capture_item]*/

               {||
                   {copies:
                        s.read_rec_field("copies", 0u,
                                         {|| deserialize_124(s) }),
                    moves:
                        s.read_rec_field("moves", 1u,
                                         {|| deserialize_124(s) }),}
               })

}
/*@syntax::ast::capture_clause*/
fn deserialize_122<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::capture_clause {

    s.read_box(/*syntax::ast::capture_clause*/{|| @deserialize_123(s) })

}
/*syntax::ast::expr_check_mode*/
fn deserialize_128<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::expr_check_mode {
    s.read_enum("syntax::ast::expr_check_mode",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::claimed_expr }
                                              1u { syntax::ast::checked_expr }
                                            }
                                        })
                })
}
/*syntax::ast::expr_*/
fn deserialize_72<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::expr_ {
    s.read_enum("syntax::ast::expr_",
                /*[@syntax::ast::expr]*//*syntax::ast::mutability*/

                /*[syntax::ast::field]*/
                /*core::option::t<@syntax::ast::expr>*/

                /*@syntax::ast::expr*//*[@syntax::ast::expr]*//*bool*/

                /*[@syntax::ast::expr]*/

                /*@syntax::ast::expr*/
                /*[core::option::t<@syntax::ast::expr>]*/

                /*syntax::ast::binop*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*/

                /*syntax::ast::unop*//*@syntax::ast::expr*/

                /*@syntax::ast::lit*/

                /*@syntax::ast::expr*//*@syntax::ast::ty*/

                /*@syntax::ast::expr*//*syntax::ast::blk*/
                /*core::option::t<@syntax::ast::expr>*/

                /*@syntax::ast::expr*//*syntax::ast::blk*/

                /*@syntax::ast::local*//*@syntax::ast::expr*/
                /*syntax::ast::blk*/

                /*syntax::ast::blk*//*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*[syntax::ast::arm]*/
                /*syntax::ast::alt_mode*/

                /*syntax::ast::proto*//*syntax::ast::fn_decl*/
                /*syntax::ast::blk*//*@syntax::ast::capture_clause*/

                /*syntax::ast::fn_decl*//*syntax::ast::blk*/

                /*syntax::ast::blk*/

                /*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*@syntax::ast::expr*/

                /*syntax::ast::binop*//*@syntax::ast::expr*/
                /*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*syntax::ast::ident*/
                /*[@syntax::ast::ty]*/

                /*@syntax::ast::expr*//*@syntax::ast::expr*/

                /*@syntax::ast::path*/

                /*core::option::t<@syntax::ast::expr>*/





                /*core::option::t<@syntax::ast::expr>*/

                /*@syntax::ast::expr*/

                /*int*//*@syntax::ast::expr*//*@syntax::ast::expr*/

                /*@syntax::ast::expr*/

                /*syntax::ast::expr_check_mode*//*@syntax::ast::expr*/

                /*@syntax::ast::expr*//*syntax::ast::blk*/
                /*core::option::t<@syntax::ast::expr>*/

                /*syntax::ast::mac*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::expr_vec(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_73(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_33(s)
                                                                                              }))
                                              }
                                              1u {
                                                syntax::ast::expr_rec(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_74(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_77(s)
                                                                                              }))
                                              }
                                              2u {
                                                syntax::ast::expr_call(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_73(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(2u,
                                                                                               {||
                                                                                                   deserialize_18(s)
                                                                                               }))
                                              }
                                              3u {
                                                syntax::ast::expr_tup(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_73(s)
                                                                                              }))
                                              }
                                              4u {
                                                syntax::ast::expr_bind(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_78(s)
                                                                                               }))
                                              }
                                              5u {
                                                syntax::ast::expr_binary(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_79(s)
                                                                                                 }),
                                                                         s.read_enum_variant_arg(1u,
                                                                                                 {||
                                                                                                     deserialize_70(s)
                                                                                                 }),
                                                                         s.read_enum_variant_arg(2u,
                                                                                                 {||
                                                                                                     deserialize_70(s)
                                                                                                 }))
                                              }
                                              6u {
                                                syntax::ast::expr_unary(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_80(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }))
                                              }
                                              7u {
                                                syntax::ast::expr_lit(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_58(s)
                                                                                              }))
                                              }
                                              8u {
                                                syntax::ast::expr_cast(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_29(s)
                                                                                               }))
                                              }
                                              9u {
                                                syntax::ast::expr_if(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_70(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_81(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(2u,
                                                                                             {||
                                                                                                 deserialize_77(s)
                                                                                             }))
                                              }
                                              10u {
                                                syntax::ast::expr_while(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_81(s)
                                                                                                }))
                                              }
                                              11u {
                                                syntax::ast::expr_for(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_104(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_70(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(2u,
                                                                                              {||
                                                                                                  deserialize_81(s)
                                                                                              }))
                                              }
                                              12u {
                                                syntax::ast::expr_do_while(s.read_enum_variant_arg(0u,
                                                                                                   {||
                                                                                                       deserialize_81(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(1u,
                                                                                                   {||
                                                                                                       deserialize_70(s)
                                                                                                   }))
                                              }
                                              13u {
                                                syntax::ast::expr_alt(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_70(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_119(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(2u,
                                                                                              {||
                                                                                                  deserialize_121(s)
                                                                                              }))
                                              }
                                              14u {
                                                syntax::ast::expr_fn(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_37(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_38(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(2u,
                                                                                             {||
                                                                                                 deserialize_81(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(3u,
                                                                                             {||
                                                                                                 deserialize_122(s)
                                                                                             }))
                                              }
                                              15u {
                                                syntax::ast::expr_fn_block(s.read_enum_variant_arg(0u,
                                                                                                   {||
                                                                                                       deserialize_38(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(1u,
                                                                                                   {||
                                                                                                       deserialize_81(s)
                                                                                                   }))
                                              }
                                              16u {
                                                syntax::ast::expr_block(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_81(s)
                                                                                                }))
                                              }
                                              17u {
                                                syntax::ast::expr_copy(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }))
                                              }
                                              18u {
                                                syntax::ast::expr_move(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }))
                                              }
                                              19u {
                                                syntax::ast::expr_assign(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_70(s)
                                                                                                 }),
                                                                         s.read_enum_variant_arg(1u,
                                                                                                 {||
                                                                                                     deserialize_70(s)
                                                                                                 }))
                                              }
                                              20u {
                                                syntax::ast::expr_swap(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_70(s)
                                                                                               }))
                                              }
                                              21u {
                                                syntax::ast::expr_assign_op(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_79(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(1u,
                                                                                                    {||
                                                                                                        deserialize_70(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(2u,
                                                                                                    {||
                                                                                                        deserialize_70(s)
                                                                                                    }))
                                              }
                                              22u {
                                                syntax::ast::expr_field(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_1(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(2u,
                                                                                                {||
                                                                                                    deserialize_53(s)
                                                                                                }))
                                              }
                                              23u {
                                                syntax::ast::expr_index(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }))
                                              }
                                              24u {
                                                syntax::ast::expr_path(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_49(s)
                                                                                               }))
                                              }
                                              25u {
                                                syntax::ast::expr_fail(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_77(s)
                                                                                               }))
                                              }
                                              26u { syntax::ast::expr_break }
                                              27u { syntax::ast::expr_cont }
                                              28u {
                                                syntax::ast::expr_ret(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_77(s)
                                                                                              }))
                                              }
                                              29u {
                                                syntax::ast::expr_be(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_70(s)
                                                                                             }))
                                              }
                                              30u {
                                                syntax::ast::expr_log(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_127(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_70(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(2u,
                                                                                              {||
                                                                                                  deserialize_70(s)
                                                                                              }))
                                              }
                                              31u {
                                                syntax::ast::expr_assert(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_70(s)
                                                                                                 }))
                                              }
                                              32u {
                                                syntax::ast::expr_check(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_128(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }))
                                              }
                                              33u {
                                                syntax::ast::expr_if_check(s.read_enum_variant_arg(0u,
                                                                                                   {||
                                                                                                       deserialize_70(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(1u,
                                                                                                   {||
                                                                                                       deserialize_81(s)
                                                                                                   }),
                                                                           s.read_enum_variant_arg(2u,
                                                                                                   {||
                                                                                                       deserialize_77(s)
                                                                                                   }))
                                              }
                                              34u {
                                                syntax::ast::expr_mac(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_67(s)
                                                                                              }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::expr*/
fn deserialize_71<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::expr {

    s.read_rec(


               /*syntax::ast::node_id*/

               /*syntax::ast::expr_*/

               /*syntax::codemap::span*/

               {||
                   {id: s.read_rec_field("id", 0u, {|| deserialize_27(s) }),
                    node:
                        s.read_rec_field("node", 1u, {|| deserialize_72(s) }),
                    span:
                        s.read_rec_field("span", 2u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::expr*/
fn deserialize_70<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::expr {

    s.read_box(/*syntax::ast::expr*/{|| @deserialize_71(s) })

}
/*syntax::ast::mac_arg<@syntax::ast::expr>*/
fn deserialize_69<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mac_arg<@syntax::ast::expr> {
    s.read_enum("core::option::t",


                /*@syntax::ast::expr*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_70(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::mac_body_*/
fn deserialize_130<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mac_body_ {

    s.read_rec(


               /*syntax::codemap::span*/

               {||
                   {span:
                        s.read_rec_field("span", 0u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::mac_body<syntax::ast::mac_body_>*/
fn deserialize_129<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mac_body<syntax::ast::mac_body_> {
    s.read_enum("core::option::t",


                /*syntax::ast::mac_body_*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_130(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::mac_*/
fn deserialize_68<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mac_ {
    s.read_enum("syntax::ast::mac_",
                /*@syntax::ast::path*/
                /*syntax::ast::mac_arg<@syntax::ast::expr>*/
                /*syntax::ast::mac_body<syntax::ast::mac_body_>*/

                /*@syntax::ast::ty*/

                /*syntax::ast::blk*/



                /*syntax::codemap::span*//*@syntax::ast::expr*/

                /*uint*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::mac_invoc(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_49(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_69(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(2u,
                                                                                               {||
                                                                                                   deserialize_129(s)
                                                                                               }))
                                              }
                                              1u {
                                                syntax::ast::mac_embed_type(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_29(s)
                                                                                                    }))
                                              }
                                              2u {
                                                syntax::ast::mac_embed_block(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_81(s)
                                                                                                     }))
                                              }
                                              3u { syntax::ast::mac_ellipsis }
                                              4u {
                                                syntax::ast::mac_aq(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_19(s)
                                                                                            }),
                                                                    s.read_enum_variant_arg(1u,
                                                                                            {||
                                                                                                deserialize_70(s)
                                                                                            }))
                                              }
                                              5u {
                                                syntax::ast::mac_var(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_20(s)
                                                                                             }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::mac*/
fn deserialize_67<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::mac {

    s.read_rec(


               /*syntax::ast::mac_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_68(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::ty_*/
fn deserialize_31<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_ {
    s.read_enum("syntax::ast::ty_",




                /*syntax::ast::mt*/

                /*syntax::ast::mt*/

                /*syntax::ast::mt*/

                /*syntax::ast::mt*/

                /*[syntax::ast::ty_field]*/

                /*syntax::ast::proto*//*syntax::ast::fn_decl*/

                /*[@syntax::ast::ty]*/

                /*@syntax::ast::path*//*syntax::ast::node_id*/

                /*@syntax::ast::ty*//*[@syntax::ast::ty_constr]*/

                /*syntax::ast::mac*/


                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::ty_nil }
                                              1u { syntax::ast::ty_bot }
                                              2u {
                                                syntax::ast::ty_box(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_32(s)
                                                                                            }))
                                              }
                                              3u {
                                                syntax::ast::ty_uniq(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_32(s)
                                                                                             }))
                                              }
                                              4u {
                                                syntax::ast::ty_vec(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_32(s)
                                                                                            }))
                                              }
                                              5u {
                                                syntax::ast::ty_ptr(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_32(s)
                                                                                            }))
                                              }
                                              6u {
                                                syntax::ast::ty_rec(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_34(s)
                                                                                            }))
                                              }
                                              7u {
                                                syntax::ast::ty_fn(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_37(s)
                                                                                           }),
                                                                   s.read_enum_variant_arg(1u,
                                                                                           {||
                                                                                               deserialize_38(s)
                                                                                           }))
                                              }
                                              8u {
                                                syntax::ast::ty_tup(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_53(s)
                                                                                            }))
                                              }
                                              9u {
                                                syntax::ast::ty_path(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_49(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_27(s)
                                                                                             }))
                                              }
                                              10u {
                                                syntax::ast::ty_constr(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_29(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_59(s)
                                                                                               }))
                                              }
                                              11u {
                                                syntax::ast::ty_mac(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_67(s)
                                                                                            }))
                                              }
                                              12u { syntax::ast::ty_infer }
                                            }
                                        })
                })
}
/*syntax::ast::ty*/
fn deserialize_30<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty {

    s.read_rec(


               /*syntax::ast::ty_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u, {|| deserialize_31(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::ty*/
fn deserialize_29<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::ty {

    s.read_box(/*syntax::ast::ty*/{|| @deserialize_30(s) })

}
/*syntax::ast::ty_param_bound*/
fn deserialize_135<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_param_bound {
    s.read_enum("syntax::ast::ty_param_bound",




                /*@syntax::ast::ty*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::bound_copy }
                                              1u { syntax::ast::bound_send }
                                              2u {
                                                syntax::ast::bound_iface(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_29(s)
                                                                                                 }))
                                              }
                                            }
                                        })
                })
}
/*[syntax::ast::ty_param_bound]*/
fn deserialize_134<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::ty_param_bound] {
    s.read_vec(

               /*syntax::ast::ty_param_bound*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_135(s) })
                                })
               })
}
/*@[syntax::ast::ty_param_bound]*/
fn deserialize_133<S: std::serialization::deserializer>(s: S) ->
   @[syntax::ast::ty_param_bound] {

    s.read_box(/*[syntax::ast::ty_param_bound]*/{|| @deserialize_134(s) })

}
/*syntax::ast::ty_param*/
fn deserialize_132<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_param {

    s.read_rec(


               /*syntax::ast::ident*/

               /*syntax::ast::node_id*/

               /*@[syntax::ast::ty_param_bound]*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    id: s.read_rec_field("id", 1u, {|| deserialize_27(s) }),
                    bounds:
                        s.read_rec_field("bounds", 2u,
                                         {|| deserialize_133(s) }),}
               })

}
/*[syntax::ast::ty_param]*/
fn deserialize_131<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::ty_param] {
    s.read_vec(

               /*syntax::ast::ty_param*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_132(s) })
                                })
               })
}
/*[@syntax::ast::item]*/
fn deserialize_137<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::item] {
    s.read_vec(

               /*@syntax::ast::item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_117(s) })
                                })
               })
}
/*syntax::ast::_mod*/
fn deserialize_136<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::_mod {

    s.read_rec(


               /*[@syntax::ast::view_item]*/

               /*[@syntax::ast::item]*/

               {||
                   {view_items:
                        s.read_rec_field("view_items", 0u,
                                         {|| deserialize_83(s) }),
                    items:
                        s.read_rec_field("items", 1u,
                                         {|| deserialize_137(s) }),}
               })

}
/*syntax::ast::native_item_*/
fn deserialize_142<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::native_item_ {
    s.read_enum("syntax::ast::native_item_",
                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::native_item_fn(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_38(s)
                                                                                                    }),
                                                                            s.read_enum_variant_arg(1u,
                                                                                                    {||
                                                                                                        deserialize_131(s)
                                                                                                    }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::native_item*/
fn deserialize_141<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::native_item {

    s.read_rec(


               /*syntax::ast::ident*/

               /*[syntax::ast::attribute]*/

               /*syntax::ast::native_item_*/

               /*syntax::ast::node_id*/

               /*syntax::codemap::span*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    attrs:
                        s.read_rec_field("attrs", 1u, {|| deserialize_2(s) }),
                    node:
                        s.read_rec_field("node", 2u,
                                         {|| deserialize_142(s) }),
                    id: s.read_rec_field("id", 3u, {|| deserialize_27(s) }),
                    span:
                        s.read_rec_field("span", 4u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::native_item*/
fn deserialize_140<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::native_item {

    s.read_box(/*syntax::ast::native_item*/{|| @deserialize_141(s) })

}
/*[@syntax::ast::native_item]*/
fn deserialize_139<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::native_item] {
    s.read_vec(

               /*@syntax::ast::native_item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_140(s) })
                                })
               })
}
/*syntax::ast::native_mod*/
fn deserialize_138<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::native_mod {

    s.read_rec(


               /*[@syntax::ast::view_item]*/

               /*[@syntax::ast::native_item]*/

               {||
                   {view_items:
                        s.read_rec_field("view_items", 0u,
                                         {|| deserialize_83(s) }),
                    items:
                        s.read_rec_field("items", 1u,
                                         {|| deserialize_139(s) }),}
               })

}
/*syntax::ast::variant_arg*/
fn deserialize_147<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::variant_arg {

    s.read_rec(


               /*@syntax::ast::ty*/

               /*syntax::ast::node_id*/

               {||
                   {ty: s.read_rec_field("ty", 0u, {|| deserialize_29(s) }),
                    id: s.read_rec_field("id", 1u, {|| deserialize_27(s) }),}
               })
}
/*[syntax::ast::variant_arg]*/
fn deserialize_146<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::variant_arg] {
    s.read_vec(

               /*syntax::ast::variant_arg*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_147(s) })
                                })
               })
}
/*syntax::ast::variant_*/
fn deserialize_145<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::variant_ {

    s.read_rec(


               /*syntax::ast::ident*/

               /*[syntax::ast::attribute]*/

               /*[syntax::ast::variant_arg]*/

               /*syntax::ast::node_id*/

               /*core::option::t<@syntax::ast::expr>*/

               {||
                   {name:
                        s.read_rec_field("name", 0u, {|| deserialize_1(s) }),
                    attrs:
                        s.read_rec_field("attrs", 1u, {|| deserialize_2(s) }),
                    args:
                        s.read_rec_field("args", 2u,
                                         {|| deserialize_146(s) }),
                    id: s.read_rec_field("id", 3u, {|| deserialize_27(s) }),
                    disr_expr:
                        s.read_rec_field("disr_expr", 4u,
                                         {|| deserialize_77(s) }),}
               })

}
/*syntax::ast::variant*/
fn deserialize_144<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::variant {

    s.read_rec(


               /*syntax::ast::variant_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u,
                                         {|| deserialize_145(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::variant]*/
fn deserialize_143<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::variant] {
    s.read_vec(

               /*syntax::ast::variant*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_144(s) })
                                })
               })
}
/*syntax::ast::privacy*/
fn deserialize_152<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::privacy {
    s.read_enum("syntax::ast::privacy",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { syntax::ast::priv }
                                              1u { syntax::ast::pub }
                                            }
                                        })
                })
}
/*syntax::ast::class_mutability*/
fn deserialize_154<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_mutability {
    s.read_enum("syntax::ast::class_mutability",



                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::class_mutable
                                              }
                                              1u {
                                                syntax::ast::class_immutable
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::class_member*/
fn deserialize_153<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_member {
    s.read_enum("syntax::ast::class_member",
                /*syntax::ast::ident*//*@syntax::ast::ty*/
                /*syntax::ast::class_mutability*//*syntax::ast::node_id*/

                /*@syntax::ast::item*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::instance_var(s.read_enum_variant_arg(0u,
                                                                                                  {||
                                                                                                      deserialize_1(s)
                                                                                                  }),
                                                                          s.read_enum_variant_arg(1u,
                                                                                                  {||
                                                                                                      deserialize_29(s)
                                                                                                  }),
                                                                          s.read_enum_variant_arg(2u,
                                                                                                  {||
                                                                                                      deserialize_154(s)
                                                                                                  }),
                                                                          s.read_enum_variant_arg(3u,
                                                                                                  {||
                                                                                                      deserialize_27(s)
                                                                                                  }))
                                              }
                                              1u {
                                                syntax::ast::class_method(s.read_enum_variant_arg(0u,
                                                                                                  {||
                                                                                                      deserialize_117(s)
                                                                                                  }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::class_item_*/
fn deserialize_151<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_item_ {

    s.read_rec(


               /*syntax::ast::privacy*/

               /*syntax::ast::class_member*/

               {||
                   {privacy:
                        s.read_rec_field("privacy", 0u,
                                         {|| deserialize_152(s) }),
                    decl:
                        s.read_rec_field("decl", 1u,
                                         {|| deserialize_153(s) }),}
               })

}
/*syntax::ast::class_item*/
fn deserialize_150<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_item {

    s.read_rec(


               /*syntax::ast::class_item_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u,
                                         {|| deserialize_151(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::class_item*/
fn deserialize_149<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::class_item {

    s.read_box(/*syntax::ast::class_item*/{|| @deserialize_150(s) })

}
/*[@syntax::ast::class_item]*/
fn deserialize_148<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::class_item] {
    s.read_vec(

               /*@syntax::ast::class_item*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_149(s) })
                                })
               })
}
/*syntax::ast::class_ctor_*/
fn deserialize_156<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_ctor_ {

    s.read_rec(


               /*syntax::ast::node_id*/

               /*syntax::ast::fn_decl*/

               /*syntax::ast::blk*/

               {||
                   {id: s.read_rec_field("id", 0u, {|| deserialize_27(s) }),
                    dec: s.read_rec_field("dec", 1u, {|| deserialize_38(s) }),
                    body:
                        s.read_rec_field("body", 2u,
                                         {|| deserialize_81(s) }),}
               })

}
/*syntax::ast::class_ctor*/
fn deserialize_155<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::class_ctor {

    s.read_rec(


               /*syntax::ast::class_ctor_*/

               /*syntax::codemap::span*/

               {||
                   {node:
                        s.read_rec_field("node", 0u,
                                         {|| deserialize_156(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
/*syntax::ast::ty_method*/
fn deserialize_158<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::ty_method {

    s.read_rec(


               /*syntax::ast::ident*/

               /*[syntax::ast::attribute]*/

               /*syntax::ast::fn_decl*/

               /*[syntax::ast::ty_param]*/

               /*syntax::codemap::span*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    attrs:
                        s.read_rec_field("attrs", 1u, {|| deserialize_2(s) }),
                    decl:
                        s.read_rec_field("decl", 2u, {|| deserialize_38(s) }),
                    tps:
                        s.read_rec_field("tps", 3u, {|| deserialize_131(s) }),
                    span:
                        s.read_rec_field("span", 4u,
                                         {|| deserialize_19(s) }),}
               })

}
/*[syntax::ast::ty_method]*/
fn deserialize_157<S: std::serialization::deserializer>(s: S) ->
   [syntax::ast::ty_method] {
    s.read_vec(

               /*syntax::ast::ty_method*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_158(s) })
                                })
               })
}
/*core::option::t<@syntax::ast::ty>*/
fn deserialize_159<S: std::serialization::deserializer>(s: S) ->
   core::option::t<@syntax::ast::ty> {
    s.read_enum("core::option::t",


                /*@syntax::ast::ty*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u { core::option::none }
                                              1u {
                                                core::option::some(s.read_enum_variant_arg(0u,
                                                                                           {||
                                                                                               deserialize_29(s)
                                                                                           }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::method*/
fn deserialize_162<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::method {

    s.read_rec(


               /*syntax::ast::ident*/

               /*[syntax::ast::attribute]*/

               /*[syntax::ast::ty_param]*/

               /*syntax::ast::fn_decl*/

               /*syntax::ast::blk*/

               /*syntax::ast::node_id*/

               /*syntax::codemap::span*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    attrs:
                        s.read_rec_field("attrs", 1u, {|| deserialize_2(s) }),
                    tps:
                        s.read_rec_field("tps", 2u, {|| deserialize_131(s) }),
                    decl:
                        s.read_rec_field("decl", 3u, {|| deserialize_38(s) }),
                    body:
                        s.read_rec_field("body", 4u, {|| deserialize_81(s) }),
                    id: s.read_rec_field("id", 5u, {|| deserialize_27(s) }),
                    span:
                        s.read_rec_field("span", 6u,
                                         {|| deserialize_19(s) }),}
               })

}
/*@syntax::ast::method*/
fn deserialize_161<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::method {

    s.read_box(/*syntax::ast::method*/{|| @deserialize_162(s) })

}
/*[@syntax::ast::method]*/
fn deserialize_160<S: std::serialization::deserializer>(s: S) ->
   [@syntax::ast::method] {
    s.read_vec(

               /*@syntax::ast::method*/
               {|len|
                   vec::init_fn(len,
                                {|i|
                                    s.read_vec_elt(i,
                                                   {|| deserialize_161(s) })
                                })
               })
}
/*syntax::ast::item_*/
fn deserialize_28<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::item_ {
    s.read_enum("syntax::ast::item_",
                /*@syntax::ast::ty*//*@syntax::ast::expr*/

                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                /*syntax::ast::blk*/

                /*syntax::ast::_mod*/

                /*syntax::ast::native_mod*/

                /*@syntax::ast::ty*//*[syntax::ast::ty_param]*/

                /*[syntax::ast::variant]*//*[syntax::ast::ty_param]*/

                /*syntax::ast::fn_decl*//*[syntax::ast::ty_param]*/
                /*syntax::ast::blk*//*syntax::ast::node_id*/
                /*syntax::ast::node_id*/

                /*[syntax::ast::ty_param]*//*[@syntax::ast::class_item]*/
                /*syntax::ast::class_ctor*/

                /*[syntax::ast::ty_param]*//*[syntax::ast::ty_method]*/

                /*[syntax::ast::ty_param]*/
                /*core::option::t<@syntax::ast::ty>*//*@syntax::ast::ty*/
                /*[@syntax::ast::method]*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::item_const(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_29(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_70(s)
                                                                                                }))
                                              }
                                              1u {
                                                syntax::ast::item_fn(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_38(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_131(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(2u,
                                                                                             {||
                                                                                                 deserialize_81(s)
                                                                                             }))
                                              }
                                              2u {
                                                syntax::ast::item_mod(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_136(s)
                                                                                              }))
                                              }
                                              3u {
                                                syntax::ast::item_native_mod(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_138(s)
                                                                                                     }))
                                              }
                                              4u {
                                                syntax::ast::item_ty(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_29(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_131(s)
                                                                                             }))
                                              }
                                              5u {
                                                syntax::ast::item_enum(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_143(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_131(s)
                                                                                               }))
                                              }
                                              6u {
                                                syntax::ast::item_res(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_38(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(1u,
                                                                                              {||
                                                                                                  deserialize_131(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(2u,
                                                                                              {||
                                                                                                  deserialize_81(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(3u,
                                                                                              {||
                                                                                                  deserialize_27(s)
                                                                                              }),
                                                                      s.read_enum_variant_arg(4u,
                                                                                              {||
                                                                                                  deserialize_27(s)
                                                                                              }))
                                              }
                                              7u {
                                                syntax::ast::item_class(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_131(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_148(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(2u,
                                                                                                {||
                                                                                                    deserialize_155(s)
                                                                                                }))
                                              }
                                              8u {
                                                syntax::ast::item_iface(s.read_enum_variant_arg(0u,
                                                                                                {||
                                                                                                    deserialize_131(s)
                                                                                                }),
                                                                        s.read_enum_variant_arg(1u,
                                                                                                {||
                                                                                                    deserialize_157(s)
                                                                                                }))
                                              }
                                              9u {
                                                syntax::ast::item_impl(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_131(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_159(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(2u,
                                                                                               {||
                                                                                                   deserialize_29(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(3u,
                                                                                               {||
                                                                                                   deserialize_160(s)
                                                                                               }))
                                              }
                                            }
                                        })
                })
}
/*syntax::ast::item*/
fn deserialize_0<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::item {

    s.read_rec(


               /*syntax::ast::ident*/

               /*[syntax::ast::attribute]*/

               /*syntax::ast::node_id*/

               /*syntax::ast::item_*/

               /*syntax::codemap::span*/

               {||
                   {ident:
                        s.read_rec_field("ident", 0u, {|| deserialize_1(s) }),
                    attrs:
                        s.read_rec_field("attrs", 1u, {|| deserialize_2(s) }),
                    id: s.read_rec_field("id", 2u, {|| deserialize_27(s) }),
                    node:
                        s.read_rec_field("node", 3u, {|| deserialize_28(s) }),
                    span:
                        s.read_rec_field("span", 4u,
                                         {|| deserialize_19(s) }),}
               })

}
fn deserialize_syntax_ast_item<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::item {
    deserialize_0(s)
}
/*syntax::ast::crate_num*/
fn serialize_165<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::crate_num) {
    s.emit_int(v);
}
/*syntax::ast::def_id*/
fn serialize_164<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::def_id) {

    s.emit_rec(/*syntax::ast::crate_num*//*syntax::ast::node_id*/
               {||
                   {
                       s.emit_rec_field("crate", 0u,
                                        {|| serialize_165(s, v.crate) });
                       s.emit_rec_field("node", 1u,
                                        {|| serialize_27(s, v.node) })
                   }
               });
}
/*syntax::ast::prim_ty*/
fn serialize_166<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::prim_ty) {

    s.emit_enum("syntax::ast::prim_ty",
                /*syntax::ast::int_ty*/
                /*syntax::ast::uint_ty*/
                /*syntax::ast::float_ty*/

                {||
                    alt v {
                      syntax::ast::ty_int(v0) {
                        s.emit_enum_variant("syntax::ast::ty_int", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_14(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_uint(v0) {
                        s.emit_enum_variant("syntax::ast::ty_uint", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_16(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_float(v0) {
                        s.emit_enum_variant("syntax::ast::ty_float", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_17(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ty_str {
                        s.emit_enum_variant("syntax::ast::ty_str", 3u, 0u,
                                            {|| })
                      }
                      syntax::ast::ty_bool {
                        s.emit_enum_variant("syntax::ast::ty_bool", 4u, 0u,
                                            {|| })
                      }
                    }
                });
}
/*@syntax::ast::def*/
fn serialize_167<S: std::serialization::serializer>(s: S,
                                                    v: @syntax::ast::def) {

    s.emit_box(/*syntax::ast::def*/{|| serialize_163(s, *v) });
}
/*syntax::ast::def*/
fn serialize_163<S: std::serialization::serializer>(s: S,
                                                    v: syntax::ast::def) {

    s.emit_enum("syntax::ast::def",
                /*syntax::ast::def_id*//*syntax::ast::purity*/
                /*syntax::ast::node_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::node_id*/
                /*syntax::ast::mode<syntax::ast::rmode>*/
                /*syntax::ast::node_id*//*bool*/
                /*syntax::ast::def_id*//*syntax::ast::def_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::prim_ty*/
                /*syntax::ast::def_id*//*uint*/
                /*syntax::ast::node_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::node_id*//*@syntax::ast::def*/
                /*syntax::ast::node_id*/
                /*syntax::ast::def_id*/
                /*syntax::ast::def_id*//*syntax::ast::def_id*/
                /*syntax::ast::def_id*//*syntax::ast::def_id*/
                {||
                    alt v {
                      syntax::ast::def_fn(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_fn", 0u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_43(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_self(v0) {
                        s.emit_enum_variant("syntax::ast::def_self", 1u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_mod(v0) {
                        s.emit_enum_variant("syntax::ast::def_mod", 2u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_native_mod(v0) {
                        s.emit_enum_variant("syntax::ast::def_native_mod", 3u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_const(v0) {
                        s.emit_enum_variant("syntax::ast::def_const", 4u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_arg(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_arg", 5u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_41(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_local(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_local", 6u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_18(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_variant(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_variant", 7u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_ty(v0) {
                        s.emit_enum_variant("syntax::ast::def_ty", 8u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_prim_ty(v0) {
                        s.emit_enum_variant("syntax::ast::def_prim_ty", 9u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_166(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_ty_param(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_ty_param", 10u,
                                            2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_binding(v0) {
                        s.emit_enum_variant("syntax::ast::def_binding", 11u,
                                            1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_use(v0) {
                        s.emit_enum_variant("syntax::ast::def_use", 12u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_upvar(v0, v1, v2) {
                        s.emit_enum_variant("syntax::ast::def_upvar", 13u, 3u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_167(s,
                                                                                              v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_27(s,
                                                                                             v2)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_class(v0) {
                        s.emit_enum_variant("syntax::ast::def_class", 14u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_class_field(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_class_field",
                                            15u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::def_class_method(v0, v1) {
                        s.emit_enum_variant("syntax::ast::def_class_method",
                                            16u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
fn serialize_syntax_ast_def<S: std::serialization::serializer>(s: S,
                                                               v:
                                                                   syntax::ast::def) {
    serialize_163(s, v);
}
/*syntax::ast::crate_num*/
fn deserialize_165<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::crate_num {
    s.read_int()
}
/*syntax::ast::def_id*/
fn deserialize_164<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::def_id {

    s.read_rec(


               /*syntax::ast::crate_num*/

               /*syntax::ast::node_id*/

               {||
                   {crate:
                        s.read_rec_field("crate", 0u,
                                         {|| deserialize_165(s) }),
                    node:
                        s.read_rec_field("node", 1u,
                                         {|| deserialize_27(s) }),}
               })

}
/*syntax::ast::prim_ty*/
fn deserialize_166<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::prim_ty {
    s.read_enum("syntax::ast::prim_ty",
                /*syntax::ast::int_ty*/

                /*syntax::ast::uint_ty*/

                /*syntax::ast::float_ty*/




                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::ty_int(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_14(s)
                                                                                            }))
                                              }
                                              1u {
                                                syntax::ast::ty_uint(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_16(s)
                                                                                             }))
                                              }
                                              2u {
                                                syntax::ast::ty_float(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_17(s)
                                                                                              }))
                                              }
                                              3u { syntax::ast::ty_str }
                                              4u { syntax::ast::ty_bool }
                                            }
                                        })
                })
}
/*@syntax::ast::def*/
fn deserialize_167<S: std::serialization::deserializer>(s: S) ->
   @syntax::ast::def {

    s.read_box(/*syntax::ast::def*/{|| @deserialize_163(s) })

}
/*syntax::ast::def*/
fn deserialize_163<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::def {
    s.read_enum("syntax::ast::def",
                /*syntax::ast::def_id*//*syntax::ast::purity*/

                /*syntax::ast::node_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::node_id*/
                /*syntax::ast::mode<syntax::ast::rmode>*/

                /*syntax::ast::node_id*//*bool*/

                /*syntax::ast::def_id*//*syntax::ast::def_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::prim_ty*/

                /*syntax::ast::def_id*//*uint*/

                /*syntax::ast::node_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::node_id*//*@syntax::ast::def*/
                /*syntax::ast::node_id*/

                /*syntax::ast::def_id*/

                /*syntax::ast::def_id*//*syntax::ast::def_id*/

                /*syntax::ast::def_id*//*syntax::ast::def_id*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::def_fn(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_164(s)
                                                                                            }),
                                                                    s.read_enum_variant_arg(1u,
                                                                                            {||
                                                                                                deserialize_43(s)
                                                                                            }))
                                              }
                                              1u {
                                                syntax::ast::def_self(s.read_enum_variant_arg(0u,
                                                                                              {||
                                                                                                  deserialize_27(s)
                                                                                              }))
                                              }
                                              2u {
                                                syntax::ast::def_mod(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_164(s)
                                                                                             }))
                                              }
                                              3u {
                                                syntax::ast::def_native_mod(s.read_enum_variant_arg(0u,
                                                                                                    {||
                                                                                                        deserialize_164(s)
                                                                                                    }))
                                              }
                                              4u {
                                                syntax::ast::def_const(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_164(s)
                                                                                               }))
                                              }
                                              5u {
                                                syntax::ast::def_arg(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_27(s)
                                                                                             }),
                                                                     s.read_enum_variant_arg(1u,
                                                                                             {||
                                                                                                 deserialize_41(s)
                                                                                             }))
                                              }
                                              6u {
                                                syntax::ast::def_local(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_18(s)
                                                                                               }))
                                              }
                                              7u {
                                                syntax::ast::def_variant(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_164(s)
                                                                                                 }),
                                                                         s.read_enum_variant_arg(1u,
                                                                                                 {||
                                                                                                     deserialize_164(s)
                                                                                                 }))
                                              }
                                              8u {
                                                syntax::ast::def_ty(s.read_enum_variant_arg(0u,
                                                                                            {||
                                                                                                deserialize_164(s)
                                                                                            }))
                                              }
                                              9u {
                                                syntax::ast::def_prim_ty(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_166(s)
                                                                                                 }))
                                              }
                                              10u {
                                                syntax::ast::def_ty_param(s.read_enum_variant_arg(0u,
                                                                                                  {||
                                                                                                      deserialize_164(s)
                                                                                                  }),
                                                                          s.read_enum_variant_arg(1u,
                                                                                                  {||
                                                                                                      deserialize_20(s)
                                                                                                  }))
                                              }
                                              11u {
                                                syntax::ast::def_binding(s.read_enum_variant_arg(0u,
                                                                                                 {||
                                                                                                     deserialize_27(s)
                                                                                                 }))
                                              }
                                              12u {
                                                syntax::ast::def_use(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_164(s)
                                                                                             }))
                                              }
                                              13u {
                                                syntax::ast::def_upvar(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_167(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(2u,
                                                                                               {||
                                                                                                   deserialize_27(s)
                                                                                               }))
                                              }
                                              14u {
                                                syntax::ast::def_class(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_164(s)
                                                                                               }))
                                              }
                                              15u {
                                                syntax::ast::def_class_field(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_164(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(1u,
                                                                                                     {||
                                                                                                         deserialize_164(s)
                                                                                                     }))
                                              }
                                              16u {
                                                syntax::ast::def_class_method(s.read_enum_variant_arg(0u,
                                                                                                      {||
                                                                                                          deserialize_164(s)
                                                                                                      }),
                                                                              s.read_enum_variant_arg(1u,
                                                                                                      {||
                                                                                                          deserialize_164(s)
                                                                                                      }))
                                              }
                                            }
                                        })
                })
}
fn deserialize_syntax_ast_def<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::def {
    deserialize_163(s)
}
/*middle::typeck::method_origin*/
fn serialize_168<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        middle::typeck::method_origin) {
    s.emit_enum("middle::typeck::method_origin",
                /*syntax::ast::def_id*/
                /*syntax::ast::def_id*//*uint*//*uint*//*uint*/
                /*syntax::ast::def_id*//*uint*/
                {||
                    alt v {
                      middle::typeck::method_static(v0) {
                        s.emit_enum_variant("middle::typeck::method_static",
                                            0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      middle::typeck::method_param(v0, v1, v2, v3) {
                        s.emit_enum_variant("middle::typeck::method_param",
                                            1u, 4u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v1)
                                                                            });
                                                    s.emit_enum_variant_arg(2u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v2)
                                                                            });
                                                    s.emit_enum_variant_arg(3u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v3)
                                                                            })
                                                }
                                            })
                      }
                      middle::typeck::method_iface(v0, v1) {
                        s.emit_enum_variant("middle::typeck::method_iface",
                                            2u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_20(s,
                                                                                             v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
fn serialize_middle_typeck_method_origin<S: std::serialization::serializer>(s:
                                                                                S,
                                                                            v:
                                                                                middle::typeck::method_origin) {
    serialize_168(s, v);
}
/*middle::typeck::method_origin*/
fn deserialize_168<S: std::serialization::deserializer>(s: S) ->
   middle::typeck::method_origin {
    s.read_enum("middle::typeck::method_origin",
                /*syntax::ast::def_id*/

                /*syntax::ast::def_id*//*uint*//*uint*//*uint*/

                /*syntax::ast::def_id*//*uint*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                middle::typeck::method_static(s.read_enum_variant_arg(0u,
                                                                                                      {||
                                                                                                          deserialize_164(s)
                                                                                                      }))
                                              }
                                              1u {
                                                middle::typeck::method_param(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_164(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(1u,
                                                                                                     {||
                                                                                                         deserialize_20(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(2u,
                                                                                                     {||
                                                                                                         deserialize_20(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(3u,
                                                                                                     {||
                                                                                                         deserialize_20(s)
                                                                                                     }))
                                              }
                                              2u {
                                                middle::typeck::method_iface(s.read_enum_variant_arg(0u,
                                                                                                     {||
                                                                                                         deserialize_164(s)
                                                                                                     }),
                                                                             s.read_enum_variant_arg(1u,
                                                                                                     {||
                                                                                                         deserialize_20(s)
                                                                                                     }))
                                              }
                                            }
                                        })
                })
}
fn deserialize_middle_typeck_method_origin<S: std::serialization::deserializer>(s:
                                                                                    S)
   -> middle::typeck::method_origin {
    deserialize_168(s)
}
/*middle::freevars::freevar_entry*/
fn serialize_169<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        middle::freevars::freevar_entry) {
    s.emit_rec(/*syntax::ast::def*//*syntax::codemap::span*/
               {||
                   {
                       s.emit_rec_field("def", 0u,
                                        {|| serialize_163(s, v.def) });
                       s.emit_rec_field("span", 1u,
                                        {|| serialize_19(s, v.span) })
                   }
               });
}
fn serialize_middle_freevars_freevar_entry<S: std::serialization::serializer>(s:
                                                                                  S,
                                                                              v:
                                                                                  middle::freevars::freevar_entry) {
    serialize_169(s, v);
}
/*middle::freevars::freevar_entry*/
fn deserialize_169<S: std::serialization::deserializer>(s: S) ->
   middle::freevars::freevar_entry {

    s.read_rec(


               /*syntax::ast::def*/

               /*syntax::codemap::span*/

               {||
                   {def:
                        s.read_rec_field("def", 0u, {|| deserialize_163(s) }),
                    span:
                        s.read_rec_field("span", 1u,
                                         {|| deserialize_19(s) }),}
               })

}
fn deserialize_middle_freevars_freevar_entry<S: std::serialization::deserializer>(s:
                                                                                      S)
   -> middle::freevars::freevar_entry {
    deserialize_169(s)
}
fn serialize_syntax_ast_def_id<S: std::serialization::serializer>(s: S,
                                                                  v:
                                                                      syntax::ast::def_id) {
    serialize_164(s, v);
}
fn deserialize_syntax_ast_def_id<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::def_id {
    deserialize_164(s)
}
/*syntax::ast::inlined_item*/
fn serialize_170<S: std::serialization::serializer>(s: S,
                                                    v:
                                                        syntax::ast::inlined_item) {
    s.emit_enum("syntax::ast::inlined_item",
                /*@syntax::ast::item*/
                /*syntax::ast::def_id*//*@syntax::ast::method*/
                {||
                    alt v {
                      syntax::ast::ii_item(v0) {
                        s.emit_enum_variant("syntax::ast::ii_item", 0u, 1u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_117(s,
                                                                                              v0)
                                                                            })
                                                }
                                            })
                      }
                      syntax::ast::ii_method(v0, v1) {
                        s.emit_enum_variant("syntax::ast::ii_method", 1u, 2u,
                                            {||
                                                {
                                                    s.emit_enum_variant_arg(0u,
                                                                            {||
                                                                                serialize_164(s,
                                                                                              v0)
                                                                            });
                                                    s.emit_enum_variant_arg(1u,
                                                                            {||
                                                                                serialize_161(s,
                                                                                              v1)
                                                                            })
                                                }
                                            })
                      }
                    }
                });
}
fn serialize_syntax_ast_inlined_item<S: std::serialization::serializer>(s: S,
                                                                        v:
                                                                            syntax::ast::inlined_item) {
    serialize_170(s, v);
}
/*syntax::ast::inlined_item*/
fn deserialize_170<S: std::serialization::deserializer>(s: S) ->
   syntax::ast::inlined_item {
    s.read_enum("syntax::ast::inlined_item",
                /*@syntax::ast::item*/

                /*syntax::ast::def_id*//*@syntax::ast::method*/
                {||
                    s.read_enum_variant({|v_id|
                                            alt check v_id {
                                              0u {
                                                syntax::ast::ii_item(s.read_enum_variant_arg(0u,
                                                                                             {||
                                                                                                 deserialize_117(s)
                                                                                             }))
                                              }
                                              1u {
                                                syntax::ast::ii_method(s.read_enum_variant_arg(0u,
                                                                                               {||
                                                                                                   deserialize_164(s)
                                                                                               }),
                                                                       s.read_enum_variant_arg(1u,
                                                                                               {||
                                                                                                   deserialize_161(s)
                                                                                               }))
                                              }
                                            }
                                        })
                })
}
fn deserialize_syntax_ast_inlined_item<S: std::serialization::deserializer>(s:
                                                                                S)
   -> syntax::ast::inlined_item {
    deserialize_170(s)
}
