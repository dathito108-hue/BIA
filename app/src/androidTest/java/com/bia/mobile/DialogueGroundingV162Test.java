package com.bia.mobile;

import android.test.InstrumentationTestCase;

public final class DialogueGroundingV162Test extends InstrumentationTestCase {
    @Override
    protected void setUp() throws Exception {
        super.setUp();
        MainActivity.nativeResetDialogueForTests();
    }

    private String say(String text) {
        return MainActivity.nativeChat(text, 300000, 0.9f, 0.1f, 0.1f, 2048);
    }

    public void testExplicitRepairReanchorsConversation() {
        say("Nguồn gió: gió gây ra sóng.");
        say("Nguồn mưa: mưa gây ra đường trơn.");
        say("Mưa có gây ra đường trơn không?");

        String repaired = say("Không, ý tôi là gió gây ra sóng.");
        assertTrue(repaired, repaired.contains("Hiểu rồi"));
        assertTrue(repaired, repaired.contains("gió"));
        assertTrue(repaired, repaired.contains("sóng"));

        String follow = say("Có chắc không?");
        assertTrue(follow, follow.contains("Tôi kiểm tra lại"));
    }

    public void testAmbiguousRepairAsksForRole() {
        say("Nguồn mưa: mưa gây ra mưa.");
        say("Mưa có gây ra mưa không?");
        String reply = say("Không phải mưa mà gió.");
        assertTrue(reply, reply.contains("cả hai vai"));
    }
}
