// The Rust abstract syntax tree.

import codemap::{span, filename};

type spanned<T> = {node: T, span: span};

type ident = str;

// Functions may or may not have names.
type fn_ident = option<ident>;

type path_ = {global: bool, idents: [ident], types: [@ty]};

type path = spanned<path_>;

type crate_num = int;
type node_id = int;
type def_id = {crate: crate_num, node: node_id};

const local_crate: crate_num = 0;
const crate_node_id: node_id = 0;

enum ty_param_bound {
    bound_copy,
    bound_send,
    bound_iface(@ty),
}

type ty_param = {ident: ident, id: node_id, bounds: @[ty_param_bound]};

enum def {
    def_fn(def_id, purity),
    def_self(node_id),
    def_mod(def_id),
    def_native_mod(def_id),
    def_const(def_id),
    def_arg(node_id, mode),
    def_local(node_id, bool /* is_mutbl */),
    def_variant(def_id /* enum */, def_id /* variant */),
    def_ty(def_id),
    def_prim_ty(prim_ty),
    def_ty_param(def_id, uint),
    def_binding(node_id),
    def_use(def_id),
    def_upvar(node_id /* local id of closed over var */,
              @def    /* closed over def */,
              node_id /* expr node that creates the closure */),
    def_class(def_id),
    // first def_id is for parent class
    def_class_field(def_id, def_id),
    // No purity allowed for now, I guess
    // (simpler this way, b/c presumably methods read mutable state)
    def_class_method(def_id, def_id)
}

// The set of meta_items that define the compilation environment of the crate,
// used to drive conditional compilation
type crate_cfg = [@meta_item];

type crate = spanned<crate_>;

type crate_ =
    {directives: [@crate_directive],
     module: _mod,
     attrs: [attribute],
     config: crate_cfg};

enum crate_directive_ {
    cdir_src_mod(ident, [attribute]),
    cdir_dir_mod(ident, [@crate_directive], [attribute]),

    // NB: cdir_view_item is *not* processed by the rest of the compiler, the
    // attached view_items are sunk into the crate's module during parsing,
    // and processed (resolved, imported, etc.) there. This enum-variant
    // exists only to preserve the view items in order in case we decide to
    // pretty-print crates in the future.
    cdir_view_item(@view_item),

    cdir_syntax(@path),
}

type crate_directive = spanned<crate_directive_>;

type meta_item = spanned<meta_item_>;

enum meta_item_ {
    meta_word(ident),
    meta_list(ident, [@meta_item]),
    meta_name_value(ident, lit),
}

type blk = spanned<blk_>;

type blk_ = {view_items: [@view_item], stmts: [@stmt], expr: option<@expr>,
             id: node_id, rules: blk_check_mode};

type pat = {id: node_id, node: pat_, span: span};

type field_pat = {ident: ident, pat: @pat};

enum pat_ {
    pat_wild,
    // A pat_ident may either be a new bound variable,
    // or a nullary enum (in which case the second field
    // is none).
    // In the nullary enum case, the parser can't determine
    // which it is. The resolver determines this, and
    // records this pattern's node_id in an auxiliary
    // set (of "pat_idents that refer to nullary enums")
    pat_ident(@path, option<@pat>),
    pat_enum(@path, [@pat]),
    pat_rec([field_pat], bool),
    pat_tup([@pat]),
    pat_box(@pat),
    pat_uniq(@pat),
    pat_lit(@expr),
    pat_range(@expr, @expr),
}

enum mutability { m_mutbl, m_imm, m_const, }

enum proto {
    proto_bare,    // native fn
    proto_any,     // fn
    proto_uniq,    // fn~
    proto_box,     // fn@
    proto_block,   // fn&
}

pure fn is_blockish(p: ast::proto) -> bool {
    alt p {
      proto_any | proto_block { true }
      proto_bare | proto_uniq | proto_box { false }
    }
}

enum binop {
    add,
    subtract,
    mul,
    div,
    rem,
    and,
    or,
    bitxor,
    bitand,
    bitor,
    lsl,
    lsr,
    asr,
    eq,
    lt,
    le,
    ne,
    ge,
    gt,
}

enum unop {
    box(mutability),
    uniq(mutability),
    deref, not, neg,
}

// Generally, after typeck you can get the inferred value
// using ty::resolved_T(...).
enum inferable<T> {
    expl(T), infer(node_id)
}

// "resolved" mode: the real modes.
enum rmode { by_ref, by_val, by_mutbl_ref, by_move, by_copy }

// inferable mode.
type mode = inferable<rmode>;

type stmt = spanned<stmt_>;

enum stmt_ {
    stmt_decl(@decl, node_id),

    // expr without trailing semi-colon (must have unit type):
    stmt_expr(@expr, node_id),

    // expr with trailing semi-colon (may have any type):
    stmt_semi(@expr, node_id),
}

enum init_op { init_assign, init_move, }

type initializer = {op: init_op, expr: @expr};

type local_ =  // FIXME: should really be a refinement on pat
    {is_mutbl: bool, ty: @ty, pat: @pat,
     init: option<initializer>, id: node_id};

type local = spanned<local_>;

type decl = spanned<decl_>;

enum decl_ { decl_local([@local]), decl_item(@item), }

type arm = {pats: [@pat], guard: option<@expr>, body: blk};

type field_ = {mutbl: mutability, ident: ident, expr: @expr};

type field = spanned<field_>;

enum blk_check_mode { default_blk, unchecked_blk, unsafe_blk, }

enum expr_check_mode { claimed_expr, checked_expr, }

type expr = {id: node_id, node: expr_, span: span};

enum alt_mode { alt_check, alt_exhaustive, }

enum expr_ {
    expr_vec([@expr], mutability),
    expr_rec([field], option<@expr>),
    expr_call(@expr, [@expr], bool),
    expr_tup([@expr]),
    expr_bind(@expr, [option<@expr>]),
    expr_binary(binop, @expr, @expr),
    expr_unary(unop, @expr),
    expr_lit(@lit),
    expr_cast(@expr, @ty),
    expr_if(@expr, blk, option<@expr>),
    expr_while(@expr, blk),
    expr_for(@local, @expr, blk),
    expr_do_while(blk, @expr),
    expr_alt(@expr, [arm], alt_mode),
    expr_fn(proto, fn_decl, blk, @capture_clause),
    expr_fn_block(fn_decl, blk),
    expr_block(blk),

    /*
     * FIXME: many of these @exprs should be constrained with
     * is_lval once we have constrained types working.
     */
    expr_copy(@expr),
    expr_move(@expr, @expr),
    expr_assign(@expr, @expr),
    expr_swap(@expr, @expr),
    expr_assign_op(binop, @expr, @expr),
    expr_field(@expr, ident, [@ty]),
    expr_index(@expr, @expr),
    expr_path(@path),
    expr_fail(option<@expr>),
    expr_break,
    expr_cont,
    expr_ret(option<@expr>),
    expr_be(@expr),
    expr_log(int, @expr, @expr),

    /* just an assert, no significance to typestate */
    expr_assert(@expr),

    /* preds that typestate is aware of */
    expr_check(expr_check_mode, @expr),

    /* FIXME Would be nice if expr_check desugared
       to expr_if_check. */
    expr_if_check(@expr, blk, option<@expr>),
    expr_mac(mac),
}

type capture_item = {
    id: int,
    name: ident, // Currently, can only capture a local var.
    span: span
};
type capture_clause = {
    copies: [@capture_item],
    moves: [@capture_item]
};

/*
// Says whether this is a block the user marked as
// "unchecked"
enum blk_sort {
    blk_unchecked, // declared as "exception to effect-checking rules"
    blk_checked, // all typing rules apply
}
*/

type mac = spanned<mac_>;

type mac_arg = option::t<@expr>;

type mac_body_ = {span: span};
type mac_body = option::t<mac_body_>;

enum mac_ {
    mac_invoc(@path, mac_arg, mac_body),
    mac_embed_type(@ty),
    mac_embed_block(blk),
    mac_ellipsis,
    // the span is used by the quoter/anti-quoter ...
    mac_aq(span /* span of quote */, @expr), // anti-quote
    mac_var(uint)
}

type lit = spanned<lit_>;

enum lit_ {
    lit_str(str),
    lit_int(i64, int_ty),
    lit_uint(u64, uint_ty),
    lit_float(str, float_ty),
    lit_nil,
    lit_bool(bool),
}

// NB: If you change this, you'll probably want to change the corresponding
// type structure in middle/ty.rs as well.
type mt = {ty: @ty, mutbl: mutability};

type ty_field_ = {ident: ident, mt: mt};

type ty_field = spanned<ty_field_>;

type ty_method = {ident: ident, attrs: [attribute],
                  decl: fn_decl, tps: [ty_param], span: span};

enum int_ty { ty_i, ty_char, ty_i8, ty_i16, ty_i32, ty_i64, }

enum uint_ty { ty_u, ty_u8, ty_u16, ty_u32, ty_u64, }

enum float_ty { ty_f, ty_f32, ty_f64, }

type ty = spanned<ty_>;

// Not represented directly in the AST, referred to by name through a ty_path.
enum prim_ty {
    ty_int(int_ty),
    ty_uint(uint_ty),
    ty_float(float_ty),
    ty_str,
    ty_bool,
}

enum ty_ {
    ty_nil,
    ty_bot, /* bottom type */
    ty_box(mt),
    ty_uniq(mt),
    ty_vec(mt),
    ty_ptr(mt),
    ty_rec([ty_field]),
    ty_fn(proto, fn_decl),
    ty_tup([@ty]),
    ty_path(@path, node_id),
    ty_constr(@ty, [@ty_constr]),
    ty_mac(mac),
    // ty_infer means the type should be inferred instead of it having been
    // specified. This should only appear at the "top level" of a type and not
    // nested in one.
    ty_infer,
}


/*
A constraint arg that's a function argument is referred to by its position
rather than name.  This is so we could have higher-order functions that have
constraints (potentially -- right now there's no way to write that), and also
so that the typestate pass doesn't have to map a function name onto its decl.
So, the constr_arg type is parameterized: it's instantiated with uint for
declarations, and ident for uses.
*/
enum constr_arg_general_<T> { carg_base, carg_ident(T), carg_lit(@lit), }

type fn_constr_arg = constr_arg_general_<uint>;
type sp_constr_arg<T> = spanned<constr_arg_general_<T>>;
type ty_constr_arg = sp_constr_arg<@path>;
type constr_arg = spanned<fn_constr_arg>;

// Constrained types' args are parameterized by paths, since
// we refer to paths directly and not by indices.
// The implicit root of such path, in the constraint-list for a
// constrained type, is * (referring to the base record)

type constr_general_<ARG, ID> =
    {path: @path, args: [@spanned<constr_arg_general_<ARG>>], id: ID};

// In the front end, constraints have a node ID attached.
// Typeck turns this to a def_id, using the output of resolve.
type constr_general<ARG> = spanned<constr_general_<ARG, node_id>>;
type constr_ = constr_general_<uint, node_id>;
type constr = spanned<constr_general_<uint, node_id>>;
type ty_constr_ = constr_general_<@path, node_id>;
type ty_constr = spanned<ty_constr_>;

/* The parser generates ast::constrs; resolve generates
 a mapping from each function to a list of ty::constr_defs,
 corresponding to these. */
type arg = {mode: mode, ty: @ty, ident: ident, id: node_id};

type fn_decl =
    {inputs: [arg],
     output: @ty,
     purity: purity,
     cf: ret_style,
     constraints: [@constr]};

enum purity {
    pure_fn, // declared with "pure fn"
    unsafe_fn, // declared with "unsafe fn"
    impure_fn, // declared with "fn"
    crust_fn, // declared with "crust fn"
}

enum ret_style {
    noreturn, // functions with return type _|_ that always
              // raise an error or exit (i.e. never return to the caller)
    return_val, // everything else
}

type method = {ident: ident, attrs: [attribute],
               tps: [ty_param], decl: fn_decl, body: blk,
               id: node_id, span: span};

type _mod = {view_items: [@view_item], items: [@item]};

enum native_abi {
    native_abi_rust_intrinsic,
    native_abi_cdecl,
    native_abi_stdcall,
}

type native_mod =
    {view_items: [@view_item],
     items: [@native_item]};

type variant_arg = {ty: @ty, id: node_id};

type variant_ = {name: ident, attrs: [attribute], args: [variant_arg],
                 id: node_id, disr_expr: option<@expr>};

type variant = spanned<variant_>;


// FIXME: May want to just use path here, which would allow things like
// 'import ::foo'
type simple_path = [ident];

type path_list_ident_ = {name: ident, id: node_id};
type path_list_ident = spanned<path_list_ident_>;

type view_path = spanned<view_path_>;
enum view_path_ {

    // quux = foo::bar::baz
    //
    // or just
    //
    // foo::bar::baz  (with 'baz =' implicitly on the left)
    view_path_simple(ident, @simple_path, node_id),

    // foo::bar::*
    view_path_glob(@simple_path, node_id),

    // foo::bar::{a,b,c}
    view_path_list(@simple_path, [path_list_ident], node_id)
}

type view_item = spanned<view_item_>;
enum view_item_ {
    view_item_use(ident, [@meta_item], node_id),
    view_item_import([@view_path]),
    view_item_export([@view_path])
}

// Meta-data associated with an item
type attribute = spanned<attribute_>;


// Distinguishes between attributes that decorate items and attributes that
// are contained as statements within items. These two cases need to be
// distinguished for pretty-printing.
enum attr_style { attr_outer, attr_inner, }

type attribute_ = {style: attr_style, value: meta_item};

type item = {ident: ident, attrs: [attribute],
             id: node_id, node: item_, span: span};

enum item_ {
    item_const(@ty, @expr),
    item_fn(fn_decl, [ty_param], blk),
    item_mod(_mod),
    item_native_mod(native_mod),
    item_ty(@ty, [ty_param]),
    item_enum([variant], [ty_param]),
    item_res(fn_decl /* dtor */, [ty_param], blk,
             node_id /* dtor id */, node_id /* ctor id */),
    item_class([ty_param], /* ty params for class */
               [@class_item], /* methods, etc. */
                             /* (not including ctor) */
               class_ctor
               ),
    item_iface([ty_param], [ty_method]),
    item_impl([ty_param], option<@ty> /* iface */,
              @ty /* self */, [@method]),
}

type class_item_ = {privacy: privacy, decl: class_member};
type class_item = spanned<class_item_>;

enum class_member {
    instance_var(ident, @ty, class_mutability, node_id),
    class_method(@item) // FIXME: methods aren't allowed to be
    // type-parametric.
    // without constrained types, have to duplicate some stuff. or factor out
    // item to separate out things with type params?
}

enum class_mutability { class_mutable, class_immutable }

enum privacy { priv, pub }

type class_ctor = spanned<class_ctor_>;
type class_ctor_ = {id: node_id,
                    dec: fn_decl,
                    body: blk};

type native_item =
    {ident: ident,
     attrs: [attribute],
     node: native_item_,
     id: node_id,
     span: span};

enum native_item_ {
    native_item_fn(fn_decl, [ty_param]),
}

// The data we save and restore about an inlined item or method.  This is not
// part of the AST that we parse from a file, but it becomes part of the tree
// that we trans.
enum inlined_item {
    ii_item(@item),
    ii_method(def_id /* impl id */, @method)
}

//
// Local Variables:
// mode: rust
// fill-column: 78;
// indent-tabs-mode: nil
// c-basic-offset: 4
// buffer-file-coding-system: utf-8-unix
// End:
//
