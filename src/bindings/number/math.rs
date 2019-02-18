use rlua::prelude::*;
use libm;

pub fn init(lua: &Lua) -> crate::Result<()> {
    lua.context(|lua| {
        lua.load("lua_math=math").exec()?;

        let module = lua.create_table()?;

        module.set("acos", lua.create_function(|_, num| {
            Ok(libm::acos(num))
        })?)?;

        module.set("acosf", lua.create_function(|_, num| {
            Ok(libm::acosf(num))
        })?)?;

        module.set("asin", lua.create_function(|_, num| {
            Ok(libm::asin(num))
        })?)?;

        module.set("asinf", lua.create_function(|_, num| {
            Ok(libm::asinf(num))
        })?)?;

        module.set("atan", lua.create_function(|_, num| {
            Ok(libm::atan(num))
        })?)?;

        module.set("atan2", lua.create_function(|_, (num0, num1)| {
            Ok(libm::atan2(num0, num1))
        })?)?;

        module.set("atan2f", lua.create_function(|_, (num0, num1)| {
            Ok(libm::atan2f(num0, num1))
        })?)?;

        module.set("atanf", lua.create_function(|_, num| {
            Ok(libm::atanf(num))
        })?)?;

        module.set("cbrt", lua.create_function(|_, num| {
            Ok(libm::cbrt(num))
        })?)?;

        module.set("cbrtf", lua.create_function(|_, num| {
            Ok(libm::cbrtf(num))
        })?)?;

        module.set("ceil", lua.create_function(|_, num| {
            Ok(libm::ceil(num))
        })?)?;

        module.set("ceilf", lua.create_function(|_, num| {
            Ok(libm::ceilf(num))
        })?)?;
        ;

        module.set("cos", lua.create_function(|_, num| {
            Ok(libm::cos(num))
        })?)?;

        module.set("cosf", lua.create_function(|_, num| {
            Ok(libm::cosf(num))
        })?)?;

        module.set("cosh", lua.create_function(|_, num| {
            Ok(libm::cosh(num))
        })?)?;

        module.set("coshf", lua.create_function(|_, num| {
            Ok(libm::coshf(num))
        })?)?;

        module.set("exp", lua.create_function(|_, num| {
            Ok(libm::exp(num))
        })?)?;

        module.set("exp2", lua.create_function(|_, num| {
            Ok(libm::exp2(num))
        })?)?;

        module.set("exp2f", lua.create_function(|_, num| {
            Ok(libm::exp2f(num))
        })?)?;

        module.set("expf", lua.create_function(|_, num| {
            Ok(libm::expf(num))
        })?)?;

        module.set("expm1", lua.create_function(|_, num| {
            Ok(libm::expm1(num))
        })?)?;

        module.set("expm1f", lua.create_function(|_, num| {
            Ok(libm::expm1f(num))
        })?)?;

        module.set("fabs", lua.create_function(|_, num| {
            Ok(libm::fabs(num))
        })?)?;

        module.set("fabsf", lua.create_function(|_, num| {
            Ok(libm::fabsf(num))
        })?)?;

        module.set("fdim", lua.create_function(|_, (n0, n1)| {
            Ok(libm::fdim(n0, n1))
        })?)?;

        module.set("fdimf", lua.create_function(|_, (n0, n1)| {
            Ok(libm::fdimf(n0, n1))
        })?)?;

        module.set("floor", lua.create_function(|_, num| {
            Ok(libm::floor(num))
        })?)?;

        module.set("floorf", lua.create_function(|_, num| {
            Ok(libm::floorf(num))
        })?)?;

        module.set("fma", lua.create_function(|_, (n0, n1, n2)| {
            Ok(libm::fma(n0, n1, n2))
        })?)?;

        module.set("fmaf", lua.create_function(|_, (n0, n1, n2)| {
            Ok(libm::fmaf(n0, n1, n2))
        })?)?;

        module.set("fmod", lua.create_function(|_, (n0, n1)| {
            Ok(libm::fmod(n0, n1))
        })?)?;

        module.set("fmodf", lua.create_function(|_, (n0, n1)| {
            Ok(libm::fmodf(n0, n1))
        })?)?;

        module.set("hypot", lua.create_function(|_, (n0, n1)| {
            Ok(libm::hypot(n0, n1))
        })?)?;

        module.set("hypotf", lua.create_function(|_, (n0, n1)| {
            Ok(libm::hypotf(n0, n1))
        })?)?;

        module.set("log", lua.create_function(|_, num| {
            Ok(libm::log(num))
        })?)?;

        module.set("log2", lua.create_function(|_, num| {
            Ok(libm::log2(num))
        })?)?;

        module.set("log10", lua.create_function(|_, num| {
            Ok(libm::log10(num))
        })?)?;

        module.set("log10f", lua.create_function(|_, num| {
            Ok(libm::log10f(num))
        })?)?;

        module.set("log1p", lua.create_function(|_, num| {
            Ok(libm::log1p(num))
        })?)?;

        module.set("log1pf", lua.create_function(|_, num| {
            Ok(libm::log1pf(num))
        })?)?;

        module.set("log2f", lua.create_function(|_, num| {
            Ok(libm::log2f(num))
        })?)?;

        module.set("logf", lua.create_function(|_, num| {
            Ok(libm::logf(num))
        })?)?;

        module.set("pow", lua.create_function(|_, (n0, n1)| {
            Ok(libm::pow(n0, n1))
        })?)?;

        module.set("powf", lua.create_function(|_, (n0, n1)| {
            Ok(libm::powf(n0, n1))
        })?)?;

        module.set("round", lua.create_function(|_, num| {
            Ok(libm::round(num))
        })?)?;

        module.set("roundf", lua.create_function(|_, num| {
            Ok(libm::roundf(num))
        })?)?;

        module.set("scalbn", lua.create_function(|_, (n0, n1)| {
            Ok(libm::scalbn(n0, n1))
        })?)?;

        module.set("scalbnf", lua.create_function(|_, (n0, n1)| {
            Ok(libm::scalbnf(n0, n1))
        })?)?;

        module.set("sin", lua.create_function(|_, num| {
            Ok(libm::sin(num))
        })?)?;

        module.set("sinf", lua.create_function(|_, num| {
            Ok(libm::sinf(num))
        })?)?;

        module.set("sinh", lua.create_function(|_, num| {
            Ok(libm::sinh(num))
        })?)?;

        module.set("sinhf", lua.create_function(|_, num| {
            Ok(libm::sinhf(num))
        })?)?;

        module.set("sqrt", lua.create_function(|_, num| {
            Ok(libm::sqrt(num))
        })?)?;

        module.set("sqrtf", lua.create_function(|_, num| {
            Ok(libm::sqrtf(num))
        })?)?;

        module.set("tan", lua.create_function(|_, num| {
            Ok(libm::tan(num))
        })?)?;

        module.set("tanf", lua.create_function(|_, num| {
            Ok(libm::tanf(num))
        })?)?;

        module.set("tanh", lua.create_function(|_, num| {
            Ok(libm::tanh(num))
        })?)?;

        module.set("tanhf", lua.create_function(|_, num| {
            Ok(libm::tanhf(num))
        })?)?;

        module.set("trunc", lua.create_function(|_, num| {
            Ok(libm::trunc(num))
        })?)?;

        module.set("truncf", lua.create_function(|_, num| {
            Ok(libm::truncf(num))
        })?)?;

        lua.globals().set("math", module)?;

        Ok(())
    })
}
