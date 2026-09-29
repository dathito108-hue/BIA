package com.bia.mobile;

import java.math.*;
import java.util.*;

/** Bounded wire checks; this is NOT a semantic decoder/audit of Jupiter instructions. */
final class SolanaWire {
    static final String ALPHABET="123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    static String base58(byte[] value) {
        BigInteger n=new BigInteger(1,value),base=BigInteger.valueOf(58);StringBuilder out=new StringBuilder();
        while(n.signum()>0){BigInteger[] qr=n.divideAndRemainder(base);out.append(ALPHABET.charAt(qr[1].intValue()));n=qr[0];}
        for(byte b:value){if(b!=0)break;out.append('1');}return out.reverse().toString();
    }
    static byte[] address(String s) {
        if(s==null || s.length()<32 || s.length()>44)throw new IllegalArgumentException("Địa chỉ Solana không hợp lệ");
        BigInteger n=BigInteger.ZERO;int zero=0;while(zero<s.length() && s.charAt(zero)=='1')zero++;
        for(char c:s.toCharArray()){int i=ALPHABET.indexOf(c);if(i<0)throw new IllegalArgumentException("Base58 không hợp lệ");n=n.multiply(BigInteger.valueOf(58)).add(BigInteger.valueOf(i));}
        byte[] a=n.toByteArray();int skip=a[0]==0?1:0;int size=a.length-skip;
        if(zero+size!=32)throw new IllegalArgumentException("Địa chỉ cần 32 byte");
        byte[] result=new byte[32];System.arraycopy(a,skip,result,zero,size);return result;
    }
    static BigInteger units(String text,int decimals) {
        if(text==null || !text.matches("[0-9]{1,12}(\\.[0-9]{1,"+decimals+"})?"))throw new IllegalArgumentException("Số lượng/độ chính xác không hợp lệ");
        BigInteger n=new BigDecimal(text).movePointRight(decimals).toBigIntegerExact();
        if(n.signum()<=0 || n.bitLength()>63)throw new IllegalArgumentException("Số lượng phải dương và trong giới hạn");return n;
    }
    static String display(BigInteger n,int decimals){return new BigDecimal(n,decimals).stripTrailingZeros().toPlainString();}
    static final class Transaction {
        final byte[] bytes,message;final String blockhash;
        Transaction(byte[] input,String owner) {
            // Restrict to one signer, who is also the fee payer; reject sponsored/RFQ multisignature payloads.
            if(input==null || input.length<134 || input.length>1232 || input[0]!=1)throw new IllegalArgumentException("Chỉ hỗ trợ giao dịch một người ký, tối đa 1232 byte");
            bytes=input.clone();message=Arrays.copyOfRange(bytes,65,bytes.length);
            int p=0;if((message[p]&128)!=0){if((message[p++]&255)!=128)throw new IllegalArgumentException("Phiên bản giao dịch chưa hỗ trợ");}
            if((message[p++]&255)!=1 || (message[p++]&255)!=0)throw new IllegalArgumentException("Người ký/phí không hợp lệ");p++;
            int count=message[p++]&255;if(count<1 || count>64 || p+count*32+32>message.length)throw new IllegalArgumentException("Danh sách tài khoản không hợp lệ");
            if(!Arrays.equals(address(owner),Arrays.copyOfRange(message,p,p+32)))throw new IllegalArgumentException("Ví không khớp người trả phí/người ký");
            blockhash=base58(Arrays.copyOfRange(message,p+count*32,p+count*32+32));
        }
        void unsigned(){for(int i=1;i<65;i++)if(bytes[i]!=0)throw new IllegalArgumentException("Báo giá chứa chữ ký ngoài dự kiến");}
        Transaction signed(byte[] result,String owner){Transaction next=new Transaction(result,owner);if(!Arrays.equals(message,next.message))throw new IllegalArgumentException("Ví đã thay đổi nội dung giao dịch");boolean any=false;for(int i=1;i<65;i++)any|=result[i]!=0;if(!any)throw new IllegalArgumentException("Thiếu chữ ký ví");return next;}
        String signature(){return base58(Arrays.copyOfRange(bytes,1,65));}
    }
}
