package com.bia.mobile;
final class TradingNative {
 static {System.loadLibrary("bia_core");}
 private static native String qualityCore(double[] points,long nowMs,double costBps);
 private static native String analyzeCore(double[] candles,double price,long eventMs,long nowMs,boolean connected);
 static String quality(double[] points,long nowMs,double costBps){return CoreSkills.local("market.quality",()->qualityCore(points,nowMs,costBps));}
 static String analyze(double[] candles,double price,long eventMs,long nowMs,boolean connected){return CoreSkills.local("market.analyze",()->analyzeCore(candles,price,eventMs,nowMs,connected));}
}
