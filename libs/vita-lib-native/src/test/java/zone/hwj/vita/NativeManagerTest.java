package zone.hwj.vita;

import static org.junit.jupiter.api.Assertions.*;

import java.util.Arrays;
import org.junit.jupiter.api.Test;
import zone.hwj.vita.def.NativeTest;

class NativeManagerTest {

    NativeManager nativeManager = NativeManager.getInstance();

    @Test
    void helloWorld() {
        nativeManager.helloWorld();
    }

    @Test
    void testCreateBytes() {
        byte[] bytes = nativeManager.testCreateBytes();
        System.out.println("Bytes from rust: " + Arrays.toString(bytes));
        System.out.println("String from rust: " + new String(bytes));
    }

    @Test
    void testCreateString() {
        String str = nativeManager.testCreateString();
        System.out.println("String from rust: " + str);
    }

    @Test
    void testAddInt() {
        int result = nativeManager.testAddInt(4, 898);
        System.out.println(result);
        assertEquals(4 + 898, result);
    }

    @Test
    void testStruct() {
        NativeTest testStruct = nativeManager.testStruct(1, 2, 3, 4);
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
        NativeTest testStruct = nativeManager.testStructPtr(1, 2, 3, 4);
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
}