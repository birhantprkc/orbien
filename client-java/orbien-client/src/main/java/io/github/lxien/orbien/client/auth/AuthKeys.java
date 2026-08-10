package io.github.lxien.orbien.client.auth;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.HexFormat;

public final class AuthKeys {

    private AuthKeys() {}

    public static String getAuthKey(String token, long timestamp) {
        String t = token == null ? "" : token;
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            md.update(t.getBytes(StandardCharsets.UTF_8));
            md.update(Long.toString(timestamp).getBytes(StandardCharsets.UTF_8));
            return HexFormat.of().formatHex(md.digest());
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException("MD5 not available", e);
        }
    }
}
