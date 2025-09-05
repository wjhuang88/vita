package zone.hwj.vita.mock;

import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;
import org.junit.jupiter.api.Test;

class NativeMockInvokerTest {

    NativeMockInvoker nativeMockInvoker = new NativeMockInvoker();

    @Test
    void helloWorld() {
        nativeMockInvoker.helloWorld();
    }

    @Test
    void testCreateBytes() {
        byte[] bytes = nativeMockInvoker.testCreateBytes();
        System.out.println("Bytes from rust: " + Arrays.toString(bytes));
        System.out.println("String from rust: " + new String(bytes));
    }

    @Test
    void testCreateString() {
        String str = nativeMockInvoker.testCreateString();
        System.out.println("String from rust: " + str);
    }

    @Test
    void testAddInt() {
        int result = nativeMockInvoker.testAddInt(4, 898);
        System.out.println(result);
        assertEquals(4 + 898, result);
    }

    @Test
    void testStruct() {
        NativeTest testStruct = nativeMockInvoker.testStruct(1, 2, 3, 4);
        int a = testStruct.getA();
        long b = testStruct.getB();
        float c = testStruct.getC();
        double d = testStruct.getD();
        System.out.println(a);
        System.out.println(b);
        System.out.println(c);
        System.out.println(d);
        assertEquals(1, a);
        assertEquals(2L, b);
        assertEquals(3.0f, c);
        assertEquals(4.0d, d);
    }

    @Test
    void testStructPtr() {
        NativeTest testStruct = nativeMockInvoker.testStructPtr(4, 5, 6.99f, 799999.888888888d);
        int a = testStruct.getA();
        long b = testStruct.getB();
        float c = testStruct.getC();
        double d = testStruct.getD();
        System.out.println(a);
        System.out.println(b);
        System.out.println(c);
        System.out.println(d);
        assertEquals(4, a);
        assertEquals(5L, b);
        assertEquals(6.99f, c);
        assertEquals(799999.888888888d, d);
    }
}