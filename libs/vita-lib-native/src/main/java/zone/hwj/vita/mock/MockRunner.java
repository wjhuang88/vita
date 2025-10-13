package zone.hwj.vita.mock;

import java.io.IOException;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;
import java.util.Arrays;
import reactor.core.publisher.Flux;
import zone.hwj.vita.NativeManager;
import zone.hwj.vita.api.Routes;
import zone.hwj.vita.def.NativeBuffer;
import zone.hwj.vita.tools.IoUtils;

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
        Routes routes = Routes.builder()
                .route("/123", (is, sink) -> {
                    try {
                        String resultStr = IoUtils.toUtf8String(is) + ", form java";
                        byte[] resultBytes = resultStr.getBytes(StandardCharsets.UTF_8);
                        MemorySegment resultSeg = Arena.global().allocateFrom(ValueLayout.JAVA_BYTE, resultBytes);
                        NativeBuffer response = NativeBuffer.from(Arena.global(), null, resultBytes.length, resultSeg, false);
                        sink.next(response);
                        sink.next(response);
                        sink.next(response);
                        sink.next(response);
                    } catch (IOException e) {
                        sink.error(e);
                        return;
                    }
                    sink.complete();
                })
                .build();
        manager.startServer(routes);
    }
}
