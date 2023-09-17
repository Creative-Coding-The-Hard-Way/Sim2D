(function() {var implementors = {
"nalgebra":[["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedMul.html\" title=\"trait nalgebra::ClosedMul\">ClosedMul</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Scale.html\" title=\"struct nalgebra::geometry::Scale\">Scale</a>&lt;T, D&gt;"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.DualQuaternion.html\" title=\"struct nalgebra::geometry::DualQuaternion\">DualQuaternion</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"],["impl&lt;T, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Rotation.html\" title=\"struct nalgebra::geometry::Rotation\">Rotation</a>&lt;T, D&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedAdd.html\" title=\"trait nalgebra::ClosedAdd\">ClosedAdd</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedMul.html\" title=\"trait nalgebra::ClosedMul\">ClosedMul</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.RealField.html\" title=\"trait nalgebra::RealField\">RealField</a>, C: <a class=\"trait\" href=\"nalgebra/geometry/trait.TCategory.html\" title=\"trait nalgebra::geometry::TCategory\">TCategory</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Transform.html\" title=\"struct nalgebra::geometry::Transform\">Transform</a>&lt;T, C, D&gt;<span class=\"where fmt-newline\">where\n    <a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimNameAdd.html\" title=\"trait nalgebra::base::dimension::DimNameAdd\">DimNameAdd</a>&lt;<a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;,\n    <a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;T, <a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;<a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.DimNameSum.html\" title=\"type nalgebra::base::dimension::DimNameSum\">DimNameSum</a>&lt;<a class=\"struct\" href=\"nalgebra/base/dimension/struct.Const.html\" title=\"struct nalgebra::base::dimension::Const\">Const</a>&lt;D&gt;, <a class=\"type\" href=\"nalgebra/base/dimension/type.U1.html\" title=\"type nalgebra::base::dimension::U1\">U1</a>&gt;&gt;,</span>"],["impl&lt;T, D: <a class=\"trait\" href=\"nalgebra/base/dimension/trait.DimName.html\" title=\"trait nalgebra::base::dimension::DimName\">DimName</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"type\" href=\"nalgebra/base/type.OMatrix.html\" title=\"type nalgebra::base::OMatrix\">OMatrix</a>&lt;T, D, D&gt;<span class=\"where fmt-newline\">where\n    T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedMul.html\" title=\"trait nalgebra::ClosedMul\">ClosedMul</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedAdd.html\" title=\"trait nalgebra::ClosedAdd\">ClosedAdd</a>,\n    <a class=\"struct\" href=\"nalgebra/base/default_allocator/struct.DefaultAllocator.html\" title=\"struct nalgebra::base::default_allocator::DefaultAllocator\">DefaultAllocator</a>: <a class=\"trait\" href=\"nalgebra/base/allocator/trait.Allocator.html\" title=\"trait nalgebra::base::allocator::Allocator\">Allocator</a>&lt;T, D, D&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"type\" href=\"nalgebra/geometry/type.UnitComplex.html\" title=\"type nalgebra::geometry::UnitComplex\">UnitComplex</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>, R, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Similarity.html\" title=\"struct nalgebra::geometry::Similarity\">Similarity</a>&lt;T, R, D&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,\n    R: <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T, D&gt;,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/base/trait.Scalar.html\" title=\"trait nalgebra::base::Scalar\">Scalar</a> + <a class=\"trait\" href=\"num_traits/identities/trait.Zero.html\" title=\"trait num_traits::identities::Zero\">Zero</a> + <a class=\"trait\" href=\"nalgebra/trait.ClosedAdd.html\" title=\"trait nalgebra::ClosedAdd\">ClosedAdd</a>, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Translation.html\" title=\"struct nalgebra::geometry::Translation\">Translation</a>&lt;T, D&gt;"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"type\" href=\"nalgebra/geometry/type.UnitQuaternion.html\" title=\"type nalgebra::geometry::UnitQuaternion\">UnitQuaternion</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>, R: <a class=\"trait\" href=\"nalgebra/geometry/trait.AbstractRotation.html\" title=\"trait nalgebra::geometry::AbstractRotation\">AbstractRotation</a>&lt;T, D&gt;, const D: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Isometry.html\" title=\"struct nalgebra::geometry::Isometry\">Isometry</a>&lt;T, R, D&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"nalgebra/geometry/struct.Quaternion.html\" title=\"struct nalgebra::geometry::Quaternion\">Quaternion</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"],["impl&lt;T: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"type\" href=\"nalgebra/geometry/type.UnitDualQuaternion.html\" title=\"type nalgebra::geometry::UnitDualQuaternion\">UnitDualQuaternion</a>&lt;T&gt;<span class=\"where fmt-newline\">where\n    T::<a class=\"associatedtype\" href=\"nalgebra/trait.SimdValue.html#associatedtype.Element\" title=\"type nalgebra::SimdValue::Element\">Element</a>: <a class=\"trait\" href=\"nalgebra/trait.SimdRealField.html\" title=\"trait nalgebra::SimdRealField\">SimdRealField</a>,</span>"]],
"num":[],
"num_bigint":[["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"num_bigint/struct.BigUint.html\" title=\"struct num_bigint::BigUint\">BigUint</a>"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"num_bigint/struct.BigInt.html\" title=\"struct num_bigint::BigInt\">BigInt</a>"]],
"num_complex":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_traits/trait.Num.html\" title=\"trait num_traits::Num\">Num</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"num_complex/struct.Complex.html\" title=\"struct num_complex::Complex\">Complex</a>&lt;T&gt;"]],
"num_rational":[["impl&lt;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.72.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> + <a class=\"trait\" href=\"num_integer/trait.Integer.html\" title=\"trait num_integer::Integer\">Integer</a>&gt; <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"num_rational/struct.Ratio.html\" title=\"struct num_rational::Ratio\">Ratio</a>&lt;T&gt;"]],
"num_traits":[],
"simba":[["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u32.html\">u32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i16.html\">i16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">32</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u16.html\">u16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.isize.html\">isize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i32.html\">i32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i8.html\">i8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">32</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i64.html\">i64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f32.html\">f32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f64.html\">f64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i64.html\">i64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i8.html\">i8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i16.html\">i16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u64.html\">u64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.isize.html\">isize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.isize.html\">isize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f32.html\">f32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i128.html\">i128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i32.html\">i32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i32.html\">i32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u16.html\">u16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">32</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i32.html\">i32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u128.html\">u128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i64.html\">i64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f32.html\">f32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u16.html\">u16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u32.html\">u32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u32.html\">u32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u16.html\">u16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i16.html\">i16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i8.html\">i8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u32.html\">u32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u16.html\">u16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i16.html\">i16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i128.html\">i128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i8.html\">i8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u128.html\">u128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">1</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i128.html\">i128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">1</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.WideF64x4.html\" title=\"struct simba::simd::WideF64x4\">WideF64x4</a>"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.WideF32x4.html\" title=\"struct simba::simd::WideF32x4\">WideF32x4</a>"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">32</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u8.html\">u8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.usize.html\">usize</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u128.html\">u128</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i8.html\">i8</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u64.html\">u64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f64.html\">f64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">2</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f32.html\">f32</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">16</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.WideF32x8.html\" title=\"struct simba::simd::WideF32x8\">WideF32x8</a>"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.u64.html\">u64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.i16.html\">i16</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">8</a>]&gt;"],["impl <a class=\"trait\" href=\"num_traits/identities/trait.One.html\" title=\"trait num_traits::identities::One\">One</a> for <a class=\"struct\" href=\"simba/simd/struct.AutoSimd.html\" title=\"struct simba::simd::AutoSimd\">AutoSimd</a>&lt;[<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.f64.html\">f64</a>; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.72.0/std/primitive.array.html\">4</a>]&gt;"]]
};if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()