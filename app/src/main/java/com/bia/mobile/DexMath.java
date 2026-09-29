package com.bia.mobile;
import java.math.*;

/** Exact integer V2 quote arithmetic, not a transaction simulation or signing engine. */
final class DexMath {
    static final BigInteger BPS=BigInteger.valueOf(10000);
    static BigInteger units(String amount,int decimals){
        if(decimals<0 || decimals>36 || !amount.matches("[0-9]{1,36}(\\.[0-9]{1,36})?"))throw new IllegalArgumentException("Khối lượng/decimals không hợp lệ");
        BigInteger n=new BigDecimal(amount).movePointRight(decimals).toBigIntegerExact();
        if(n.signum()<=0 || n.bitLength()>112)throw new IllegalArgumentException("Khối lượng ngoài giới hạn");return n;
    }
    static BigInteger out(BigInteger amount,BigInteger reserveIn,BigInteger reserveOut,int feeBps){
        if(amount.signum()<=0 || reserveIn.signum()<=0 || reserveOut.signum()<=0 || amount.bitLength()>112 || reserveIn.bitLength()>112 || reserveOut.bitLength()>112 || feeBps<0 || feeBps>100)throw new IllegalArgumentException("Thông số AMM không hợp lệ");
        BigInteger effective=amount.multiply(BigInteger.valueOf(10000-feeBps));
        return effective.multiply(reserveOut).divide(reserveIn.multiply(BPS).add(effective));
    }
    static BigInteger minimum(BigInteger out,int slippageBps){if(slippageBps<0 || slippageBps>100)throw new IllegalArgumentException("Slippage chỉ 0–100 bps");return out.multiply(BigInteger.valueOf(10000-slippageBps)).divide(BPS);}
    static String decimal(BigInteger value,int decimals){return new BigDecimal(value,decimals).stripTrailingZeros().toPlainString();}
}
