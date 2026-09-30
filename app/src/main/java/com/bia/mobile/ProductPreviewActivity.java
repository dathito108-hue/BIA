package com.bia.mobile;
import android.app.*;
import android.os.*;
import android.webkit.*;
import android.net.Uri;
import android.widget.*;
import java.io.*;
import java.util.*;
import java.nio.charset.StandardCharsets;

/** Only generated, verified local files. No JS bridge, file access, remote content or navigation. */
public final class ProductPreviewActivity extends Activity {
    WebView web;
    @Override public void onCreate(Bundle state){super.onCreate(state);try(ProductStore store=new ProductStore(this)){ProductBundle bundle=store.load(getIntent().getLongExtra("build",-1));show(bundle);}catch(Exception e){TextView t=new TextView(this);t.setText("Không mở được bản lưu: "+e.getMessage());setContentView(t);}}
    void show(ProductBundle bundle){web=new WebView(this);setContentView(web);WebSettings s=web.getSettings();s.setJavaScriptEnabled(true);s.setDomStorageEnabled(true);s.setAllowFileAccess(false);s.setAllowContentAccess(false);s.setBlockNetworkLoads(true);s.setMixedContentMode(WebSettings.MIXED_CONTENT_NEVER_ALLOW);s.setSupportMultipleWindows(false);web.setWebChromeClient(new WebChromeClient());web.setWebViewClient(new WebViewClient(){
        @Override public boolean shouldOverrideUrlLoading(WebView v,WebResourceRequest r){return true;}
        @Override public WebResourceResponse shouldInterceptRequest(WebView v,WebResourceRequest request){Uri u=request.getUrl();String name=u.getPath()==null?"":u.getPath().substring(1);String body="";boolean allowed="https".equals(u.getScheme())&&"bia-product.invalid".equals(u.getHost())&&"GET".equals(request.getMethod())&&Arrays.asList("index.html","style.css","app.js","core.js","config.js").contains(name);if(allowed)body=bundle.files.get(name);String mime=name.endsWith(".js")?"application/javascript":name.endsWith(".css")?"text/css":"text/html";return new WebResourceResponse(mime,"UTF-8",allowed?200:403,allowed?"OK":"Blocked",Collections.singletonMap("Cache-Control","no-store"),new ByteArrayInputStream(body.getBytes(StandardCharsets.UTF_8)));}
    });web.loadUrl("https://bia-product.invalid/index.html");}
    @Override protected void onDestroy(){if(web!=null){web.stopLoading();web.destroy();}super.onDestroy();}
}
