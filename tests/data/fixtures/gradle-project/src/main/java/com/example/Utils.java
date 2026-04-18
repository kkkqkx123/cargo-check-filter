package com.example;

import java.util.Date;
import java.util.ArrayList;
import java.util.List;

public class Utils {
    // Warning: unchecked conversion
    private static List rawList = new ArrayList();
    
    public static void processDate() {
        Date date = new Date();
        // Warning: deprecated methods
        int year = date.getYear();
        int month = date.getMonth();
        int day = date.getDate();
        
        System.out.println("Date: " + year + "-" + month + "-" + day);
    }
}
