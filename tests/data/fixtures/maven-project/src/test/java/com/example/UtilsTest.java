package com.example;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;
import java.util.Date;
import java.util.Arrays;

/**
 * Test class for Utils
 */
public class UtilsTest {

    @Test
    void testFormatDate() {
        Date date = new Date(123, 0, 15); // January 15, 2023 (year is offset from 1900)
        String result = Utils.formatDate(date);
        assertNotNull(result);
        assertTrue(result.contains("2023") || result.contains("123"));
    }

    @Test
    void testUnsafeCast() {
        String original = "test string";
        String casted = Utils.unsafeCast(original);
        assertEquals(original, casted);
    }

    @Test
    void testProcessItems() {
        // Should not throw exception
        assertDoesNotThrow(() -> {
            Utils.processItems(Arrays.asList("item1", "item2"));
        });
    }

    @Test
    void testProcessEmptyItems() {
        // Should not throw exception with empty list
        assertDoesNotThrow(() -> {
            Utils.processItems(Arrays.asList());
        });
    }
}
