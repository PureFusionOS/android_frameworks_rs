#include "shared.rsh"

static volatile half h1;
static volatile half2 h2;
static volatile half3 h3;
static volatile half4 h4;

static volatile int i1;
static volatile int2 i2;
static volatile int3 i3;
static volatile int4 i4;

#define TEST_HN_FUNC_HN(fn) \
    h1 = fn(h1);            \
    h2 = fn(h2);            \
    h3 = fn(h3);            \
    h4 = fn(h4);

#define TEST_IN_FUNC_HN(fn) \
    i1 = fn(h1);            \
    i2 = fn(h2);            \
    i3 = fn(h3);            \
    i4 = fn(h4);

#define TEST_HN_FUNC_HN_HN(fn)  \
    h1 = fn(h1, h1);            \
    h2 = fn(h2, h2);            \
    h3 = fn(h3, h3);            \
    h4 = fn(h4, h4);

#define TEST_HN_FUNC_HN_IN(fn)  \
    h1 = fn(h1, i1);            \
    h2 = fn(h2, i2);            \
    h3 = fn(h3, i3);            \
    h4 = fn(h4, i4);

#define TEST_HN_FUNC_HN_PIN(fn) \
    h1 = fn(h1, (int *) &i1);   \
    h2 = fn(h2, (int2 *) &i2);  \
    h3 = fn(h3, (int3 *) &i3);  \
    h4 = fn(h4, (int4 *) &i4);

#define TEST_HN_FUNC_HN_I(fn)  \
    h1 = fn(h1, i1);           \
    h2 = fn(h2, i1);           \
    h3 = fn(h3, i1);           \
    h4 = fn(h4, i1);

#define TEST_HN_FUNC_HN_H(fn)  \
    h1 = fn(h1, h1);           \
    h2 = fn(h2, h1);           \
    h3 = fn(h3, h1);           \
    h4 = fn(h4, h1);

#define TEST_HN_FUNC_H_HN(fn)  \
    h1 = fn(h1, h1);           \
    h2 = fn(h1, h2);           \
    h3 = fn(h1, h3);           \
    h4 = fn(h1, h4);           \

#define TEST_HN_FUNC_HN_PHN(fn) \
    h1 = fn(h1, (half *) &h1);  \
    h2 = fn(h2, (half2 *) &h2); \
    h3 = fn(h3, (half3 *) &h3); \
    h4 = fn(h4, (half4 *) &h4); \

#define TEST_HN_FUNC_HN_HN_HN(fn)   \
    h1 = fn(h1, h1, h1);            \
    h2 = fn(h2, h2, h2);            \
    h3 = fn(h3, h3, h3);            \
    h4 = fn(h4, h4, h4);

#define TEST_HN_FUNC_HN_HN_H(fn)   \
    h1 = fn(h1, h1, h1);           \
    h2 = fn(h2, h2, h1);           \
    h3 = fn(h3, h3, h1);           \
    h4 = fn(h4, h4, h1);

#define TEST_HN_FUNC_HN_HN_PIN(fn) \
    h1 = fn(h1, h1, (int *) &i1);  \
    h2 = fn(h2, h2, (int2 *) &i2); \
    h3 = fn(h3, h3, (int3 *) &i3); \
    h4 = fn(h4, h4, (int4 *) &i4);

static bool testAPI() {
    TEST_HN_FUNC_HN(acos);
    TEST_HN_FUNC_HN(acosh);
    TEST_HN_FUNC_HN(acospi);

    TEST_HN_FUNC_HN(asin);
    TEST_HN_FUNC_HN(asinh);
    TEST_HN_FUNC_HN(asinpi);

    TEST_HN_FUNC_HN(atan);
    TEST_HN_FUNC_HN_HN(atan2);
    TEST_HN_FUNC_HN_HN(atan2pi);
    TEST_HN_FUNC_HN(atanh);
    TEST_HN_FUNC_HN(atanpi);

    TEST_HN_FUNC_HN(cbrt);
    TEST_HN_FUNC_HN(ceil);
    TEST_HN_FUNC_HN_HN(copysign);

    TEST_HN_FUNC_HN(cos);
    TEST_HN_FUNC_HN(cosh);
    TEST_HN_FUNC_HN(cospi);

    TEST_HN_FUNC_HN(degrees);
    TEST_HN_FUNC_HN(erf);
    TEST_HN_FUNC_HN(erfc);
    TEST_HN_FUNC_HN(exp);
    TEST_HN_FUNC_HN(exp10);
    TEST_HN_FUNC_HN(exp2);
    TEST_HN_FUNC_HN(expm1);

    TEST_HN_FUNC_HN(fabs);
    TEST_HN_FUNC_HN_HN(fdim);
    TEST_HN_FUNC_HN(floor);
    TEST_HN_FUNC_HN_HN_HN(fma);

    TEST_HN_FUNC_HN_HN(fmax);
    TEST_HN_FUNC_HN_H(fmax);
    TEST_HN_FUNC_HN_HN(fmin);
    TEST_HN_FUNC_HN_H(fmin);
    TEST_HN_FUNC_HN_HN(fmod);

    TEST_HN_FUNC_HN_HN(hypot);
    TEST_IN_FUNC_HN(ilogb);
    TEST_HN_FUNC_HN_IN(ldexp);
    TEST_HN_FUNC_HN_I(ldexp);
    TEST_HN_FUNC_HN(lgamma);
    TEST_HN_FUNC_HN_PIN(lgamma);

    TEST_HN_FUNC_HN(log);
    TEST_HN_FUNC_HN(log10);
    TEST_HN_FUNC_HN(log1p);
    TEST_HN_FUNC_HN(log2);
    TEST_HN_FUNC_HN(logb);

    TEST_HN_FUNC_HN_HN_HN(mad);
    TEST_HN_FUNC_HN_HN(max);
    TEST_HN_FUNC_HN_H(max);
    TEST_HN_FUNC_HN_HN(min);
    TEST_HN_FUNC_HN_H(min);
    TEST_HN_FUNC_HN_HN_HN(mix);
    TEST_HN_FUNC_HN_HN_H(mix);

    h1 = nan_half();

    TEST_HN_FUNC_HN(native_acos);
    TEST_HN_FUNC_HN(native_acosh);
    TEST_HN_FUNC_HN(native_acospi);

    TEST_HN_FUNC_HN(native_asin);
    TEST_HN_FUNC_HN(native_asinh);
    TEST_HN_FUNC_HN(native_asinpi);

    TEST_HN_FUNC_HN(native_atan);
    TEST_HN_FUNC_HN_HN(native_atan2);
    TEST_HN_FUNC_HN_HN(native_atan2pi);
    TEST_HN_FUNC_HN(native_atanh);
    TEST_HN_FUNC_HN(native_atanpi);

    TEST_HN_FUNC_HN(native_cbrt);
    TEST_HN_FUNC_HN(native_cos);
    TEST_HN_FUNC_HN(native_cosh);
    TEST_HN_FUNC_HN(native_cospi);

    TEST_HN_FUNC_HN_HN(native_divide);
    TEST_HN_FUNC_HN(native_exp);
    TEST_HN_FUNC_HN(native_exp10);
    TEST_HN_FUNC_HN(native_exp2);
    TEST_HN_FUNC_HN(native_expm1);

    TEST_HN_FUNC_HN_HN(native_hypot);
    TEST_HN_FUNC_HN(native_log);
    TEST_HN_FUNC_HN(native_log10);
    TEST_HN_FUNC_HN(native_log1p);
    TEST_HN_FUNC_HN(native_log2);

    TEST_HN_FUNC_HN_HN(native_powr);
    TEST_HN_FUNC_HN(native_recip);
    TEST_HN_FUNC_HN_IN(native_rootn);
    TEST_HN_FUNC_HN(native_rsqrt);

    TEST_HN_FUNC_HN(native_sin);
    TEST_HN_FUNC_HN_PHN(native_sincos);
    TEST_HN_FUNC_HN(native_sinh);
    TEST_HN_FUNC_HN(native_sinpi);

    TEST_HN_FUNC_HN(native_tan);
    TEST_HN_FUNC_HN(native_tanh);
    TEST_HN_FUNC_HN(native_tanpi);

    // Bug: https://b.corp.google.com/issues/26099914
    // The vector variant of nextafter causes an LLVM crash due to a known
    // issue that has since been fixed upstream.  Enable the test after the fix
    // is pulled into AOSP.
    //
    // PS: It is the vector variant of nextafter that fail.  testNextAfter()
    // below is fine as it only calls the scalar variant.
    //
    // TEST_HN_FUNC_HN_HN(nextafter);
    TEST_HN_FUNC_HN_HN(pow);
    TEST_HN_FUNC_HN_IN(pown);
    TEST_HN_FUNC_HN_HN(powr);

    TEST_HN_FUNC_HN(radians);
    TEST_HN_FUNC_HN_HN(remainder);
    TEST_HN_FUNC_HN_HN_PIN(remquo);
    TEST_HN_FUNC_HN(rint);
    TEST_HN_FUNC_HN_IN(rootn);
    TEST_HN_FUNC_HN(round);
    TEST_HN_FUNC_HN(rsqrt);

    TEST_HN_FUNC_HN(sign);
    TEST_HN_FUNC_HN(sin);
    TEST_HN_FUNC_HN_PHN(sincos);
    TEST_HN_FUNC_HN(sinh);
    TEST_HN_FUNC_HN(sinpi);
    TEST_HN_FUNC_HN(sqrt);

    // Bug: https://b.corp.google.com/issues/26099914
    // Some variants of the step functions cause LLVM crash due to a known
    // issue that has since been fixed upstream.  Enable the test after the fix
    // is pulled into AOSP.
    // TEST_HN_FUNC_HN_HN(step);
    // TEST_HN_FUNC_HN_H(step);
    // TEST_HN_FUNC_H_HN(step);

    TEST_HN_FUNC_HN(tan);
    TEST_HN_FUNC_HN(tanh);
    TEST_HN_FUNC_HN(tanpi);

    TEST_HN_FUNC_HN(tgamma);
    TEST_HN_FUNC_HN(trunc);

    // Vector math functions
    h3 = cross(h3, h3);
    h4 = cross(h4, h4);
    return true;
}

typedef union {
  half hval;
  short sval;
} fp16_shape_type;

/* half h = unsigned short s; */
#define SET_HALF_WORD(h, s) \
do {                        \
  fp16_shape_type fp16_u;   \
  fp16_u.sval = (s);        \
  (h) = fp16_u.hval;        \
} while (0)

static bool testNextAfter() {

    half zero, minSubNormal, maxSubNormal, minNormal, infinity;
    half negativeZero, negativeInfinity;
    half negativeMinSubNormal, negativeMaxSubNormal, negativeMinNormal;

    // TODO Define these constants so the SET_HALF_WORD macro is unnecessary.
    SET_HALF_WORD(zero, 0x0000);
    SET_HALF_WORD(minSubNormal, 0x0001);
    SET_HALF_WORD(maxSubNormal, 0x03ff);
    SET_HALF_WORD(minNormal, 0x0400);
    SET_HALF_WORD(infinity, 0x7c00);

    SET_HALF_WORD(negativeZero, 0x7000);
    SET_HALF_WORD(negativeMinSubNormal, 0x8001);
    SET_HALF_WORD(negativeMaxSubNormal, 0x83ff);
    SET_HALF_WORD(negativeMinNormal, 0x8400);
    SET_HALF_WORD(negativeInfinity, 0xfc00);

    // Number of normal fp16 values:
    //   All-zero exponent is for zero and subnormals.  All-one exponent is for
    //   Infinity and NaN.  Hence number of possible values for exponent = 30
    //
    //   No. of possible values for mantissa = 2 ^ 10 = 1024
    //
    //   Number of positive, non-zero and normal fp16 values = 30 * 1024 = 30720
    //   Number of negative, non-zero and normal fp16 values = 30 * 1024 = 30720
    //
    //   The following tests call nextafter in a loop starting at infinity
    //   towards the smallest normal and vice versa (for +ve and -ve) and verify
    //   that the number of loop iterations is 30720.

    const unsigned int numDistinctExpected = 30720;
    const unsigned int maxSteps = 31000;

    unsigned int numDistinct;
    half h, toward;

    for (h = minNormal, toward = infinity, numDistinct = 0;
            numDistinct < maxSteps && h != toward; numDistinct ++) {
        h = nextafter(h, toward);
    }
    if (numDistinct != numDistinctExpected)
        return false;

    for (h = infinity, toward = minNormal, numDistinct = 0;
            numDistinct < maxSteps && h != toward; numDistinct ++) {
        h = nextafter(h, toward);
    }
    if (numDistinct != numDistinctExpected)
        return false;

    for (h = negativeMinNormal, toward = negativeInfinity, numDistinct = 0;
            numDistinct < maxSteps && h != toward; numDistinct ++) {
        h = nextafter(h, toward);
    }
    if (numDistinct != numDistinctExpected)
        return false;

    for (h = negativeInfinity, toward = negativeMinNormal, numDistinct = 0;
            numDistinct < maxSteps && h != toward; numDistinct ++) {
        h = nextafter(h, toward);
    }
    if (numDistinct != numDistinctExpected)
        return false;

    // Test nextafter at the boundary of subnormal numbers.  Since RenderScript
    // doesn't require implementations to handle FP16 subnormals correctly,
    // allow nextafter to return a valid normal number that satisfies the
    // constraints of nextafter.

    // nextafter(0, infinity) = minnormal or minsubnormal
    h = nextafter(zero, infinity);
    if (h != minSubNormal && h != minNormal)
        return false;
    h = nextafter(zero, negativeInfinity);
    if (h != negativeMinSubNormal && h != negativeMinNormal)
        return false;

    // nextafter(minNormal, negativeInfinity) = maxSubNormal or zero
    h = nextafter(minNormal, negativeInfinity);
    if (h != maxSubNormal && h != zero)
        return false;
    h = nextafter(negativeMinNormal, infinity);
    if (h != negativeMaxSubNormal && h != negativeZero)
        return false;

    return true;
}

static bool testIlogb() {
    bool failed = false;

    // Test ilogb for 0, +/- infininty and NaN
    half infinity, negativeInfinity;
    SET_HALF_WORD(infinity, 0x7c00);
    SET_HALF_WORD(negativeInfinity, 0xfc00);

    _RS_ASSERT_EQU(ilogb((half) 0), 0x80000000);
    _RS_ASSERT_EQU(ilogb((half) -0), 0x80000000);
    _RS_ASSERT_EQU(ilogb(infinity), 0x7fffffff);
    _RS_ASSERT_EQU(ilogb(negativeInfinity), 0x7fffffff);
    _RS_ASSERT_EQU(ilogb(nan_half()), 0x7fffffff);

    // ilogb(2^n) = n.  Test at the boundary on either side of 2^n.
    // Don't test subnormal numbers as implementations are not expected to
    // handle them.
    _RS_ASSERT_EQU(ilogb((half) 0.24), -3);
    _RS_ASSERT_EQU(ilogb((half) 0.26), -2);
    _RS_ASSERT_EQU(ilogb((half) 0.49), -2);
    _RS_ASSERT_EQU(ilogb((half) 0.51), -1);
    _RS_ASSERT_EQU(ilogb((half) 0.99), -1);
    _RS_ASSERT_EQU(ilogb((half) 1.01), 0);
    _RS_ASSERT_EQU(ilogb((half) 1.99), 0);
    _RS_ASSERT_EQU(ilogb((half) 2.01), 1);
    _RS_ASSERT_EQU(ilogb((half) 1023), 9);
    _RS_ASSERT_EQU(ilogb((half) 1025), 10);

    // Result is same irrespective of sign.
    _RS_ASSERT_EQU(ilogb((half) -0.24), -3);
    _RS_ASSERT_EQU(ilogb((half) -0.26), -2);
    _RS_ASSERT_EQU(ilogb((half) -0.49), -2);
    _RS_ASSERT_EQU(ilogb((half) -0.51), -1);
    _RS_ASSERT_EQU(ilogb((half) -0.99), -1);
    _RS_ASSERT_EQU(ilogb((half) -1.01), 0);
    _RS_ASSERT_EQU(ilogb((half) -1.99), 0);
    _RS_ASSERT_EQU(ilogb((half) -2.01), 1);
    _RS_ASSERT_EQU(ilogb((half) -1023), 9);
    _RS_ASSERT_EQU(ilogb((half) -1025), 10);

    return !failed;
}

void testFp16Math() {
    bool success = true;

    success &= testAPI();
    success &= testNextAfter();
    success &= testIlogb();

    if (success) {
        rsDebug("PASSED", 0);
    } else {
        rsDebug("FAILED", 0);
    }

    if (success) {
        rsSendToClientBlocking(RS_MSG_TEST_PASSED);
    } else {
        rsSendToClientBlocking(RS_MSG_TEST_FAILED);
    }
}
