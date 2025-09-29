package zone.hwj.vita.def;

import static org.junit.jupiter.api.Assertions.*;

import java.io.IOException;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;
import org.junit.jupiter.api.Test;
import zone.hwj.vita.tools.IoUtils;

class NativeBufferGroupTest {

    @Test
    void read() {
        try(Arena arena = Arena.ofConfined()) {
            byte[] bb1 = ("Hello ").getBytes(StandardCharsets.UTF_8);
            MemorySegment bm1 = arena.allocateFrom(ValueLayout.JAVA_BYTE, bb1);
            var b1 = NativeBuffer.fromParts(arena, null, bb1.length, bm1, false);

            byte[] bb2 = ("我的朋友🤣").getBytes(StandardCharsets.UTF_8);
            MemorySegment bm2 = arena.allocateFrom(ValueLayout.JAVA_BYTE, bb2);
            var b2 = NativeBuffer.fromParts(arena, null, bb2.length, bm2, false);

            byte[] bb3 = (", こんにちは").getBytes(StandardCharsets.UTF_8);
            MemorySegment bm3 = arena.allocateFrom(ValueLayout.JAVA_BYTE, bb3);
            var b3 = NativeBuffer.fromParts(arena, null, bb3.length, bm3, false);

            try(NativeBufferGroup group = NativeBufferGroup.of(b1)) {
                String s = IoUtils.toUtf8String(group);
                assertEquals("Hello ", s);
                System.out.println(s);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }

            try(NativeBufferGroup group = NativeBufferGroup.of(b1, b2)) {
                String s = IoUtils.toUtf8String(group);
                assertEquals("Hello 我的朋友🤣", s);
                System.out.println(s);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }

            try(NativeBufferGroup group = NativeBufferGroup.of(b1, b2, b3)) {
                String s = IoUtils.toUtf8String(group);
                assertEquals("Hello 我的朋友🤣, こんにちは", s);
                System.out.println(s);
            } catch (IOException e) {
                throw new RuntimeException(e);
            }
        }
    }
}