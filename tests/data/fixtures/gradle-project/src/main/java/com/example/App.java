package com.example;

import java.util.ArrayList;
import java.util.List;

public class App {
    public static void main(String[] args) {
        System.out.println("Hello World!");
        
        // Warning: unchecked conversion
        List rawList = new ArrayList();
        rawList.add("test");
        
        Utils.processDate();
    }
}
