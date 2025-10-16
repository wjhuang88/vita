package zone.hwj.vita.mock;

import java.util.Arrays;
import java.util.Map;
import zone.hwj.vita.NativeManager;
import zone.hwj.vita.api.RequestHandler;
import zone.hwj.vita.api.Routes;
import zone.hwj.vita.codec.BufferTranslator;
import zone.hwj.vita.codec.impl.JsonTranslator;
import zone.hwj.vita.codec.impl.StringTranslator;

public class MockRunner {
    public static void main(String[] args) {
        NativeMockInvoker nativeMockInvoker = new NativeMockInvoker();
        nativeMockInvoker.helloWorld();

        byte[] bytes = nativeMockInvoker.testCreateBytes();
        System.out.println("Bytes from rust: " + Arrays.toString(bytes));
        System.out.println("String from rust: " + new String(bytes));

        String str = nativeMockInvoker.testCreateString("dear java用户😉");
        System.out.println("String from rust: " + str);

        int result = nativeMockInvoker.testAddInt(4, 898);
        System.out.println(result);

        NativeTest testStruct = nativeMockInvoker.testStruct(1, 2, 3, 4);
        int a = testStruct.getA();
        long b = testStruct.getB();
        float c = testStruct.getC();
        double d = testStruct.getD();
        System.out.println(a);
        System.out.println(b);
        System.out.println(c);
        System.out.println(d);

        String cbr = nativeMockInvoker.testCallbackInvoke();
        System.out.println(cbr);

        NativeManager manager = NativeManager.getInstance();
        BufferTranslator<String> strTranslator = new StringTranslator();
        BufferTranslator<Object> jsonTranslator = new JsonTranslator();
        Routes routes = Routes.builder()
                .route("/123", RequestHandler.make(strTranslator, strTranslator, req -> (req + " form java!\n").repeat(3)))
                .route("/test", RequestHandler.make(jsonTranslator, jsonTranslator, req -> {
                    System.out.println(req);
                    return Map.of("testKey", "testValue", "testKey2", 123);
                }))
                .build();
        manager.startServer(routes);
    }
}
