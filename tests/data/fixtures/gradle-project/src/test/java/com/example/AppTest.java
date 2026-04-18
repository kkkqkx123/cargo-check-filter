package com.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class AppTest {
    @Test
    void testApp() {
        assertTrue(true);
    }
    
    @Test
    void testFailure() {
        // This test will fail
        assertEquals(1, 2);
    }
}
