use peroxide_ad::{
    ad_struct_def,
    ad_display,
    ad_impl,
    ad_impl_from,
    ad_iter_def,
    ad_impl_into_iter,
    ad_impl_from_iter,
    ad_impl_double_ended_iter,
    ad_impl_exact_size_iter,
    ad_impl_iter,
    ad_impl_index,
    ad_impl_neg,
    ad_impl_add,
    ad_impl_sub,
    ad_impl_mul,
    ad_impl_div,
    ad_impl_explogops,
    ad_impl_powops,
};
use crate::statistics::ops::C;
use crate::traits::num::{ExpLogOps, PowOps, TrigOps};
use std::iter::FromIterator;
use std::ops::{Neg, Add, Sub, Mul, Div, Index, IndexMut};

ad_struct_def!();
ad_display!();
ad_impl!();
ad_impl_from!();
ad_iter_def!();
ad_impl_into_iter!();
ad_impl_iter!();
ad_impl_from_iter!();
ad_impl_double_ended_iter!();
ad_impl_exact_size_iter!();
ad_impl_index!();
ad_impl_neg!();
ad_impl_add!();
ad_impl_sub!();
ad_impl_mul!();
ad_impl_div!();
ad_impl_explogops!();
ad_impl_powops!();
