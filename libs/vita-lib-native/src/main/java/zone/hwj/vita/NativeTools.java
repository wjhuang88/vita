package zone.hwj.vita;

import fr.stardustenterprises.yanl.NativeLoader;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.UUID;
import zone.hwj.vita.def.NativeBuffer;

public class NativeTools {
    public static final MethodHandle FREE_HANDLE;

    private static final Linker LINKER;
    private static final SymbolLookup LOOKUP;

    static {
        new NativeLoader.Builder().build().loadLibrary("vita", false);
        LOOKUP = SymbolLookup.loaderLookup();
        LINKER = Linker.nativeLinker();
        FREE_HANDLE = makeMethodHandle("free_jbuffer", FunctionDescriptor.ofVoid(NativeBuffer.ADDRESS_LAYOUT));
    }

    public static MethodHandle makeMethodHandle(MemorySegment methodSegment, FunctionDescriptor descriptor) {
        return LINKER.downcallHandle(methodSegment, descriptor);
    }

    public static MethodHandle makeMethodHandle(String nativeName, FunctionDescriptor descriptor) {
        return makeMethodHandle(LOOKUP.findOrThrow(nativeName), descriptor);
    }

    public static VarHandle makeVarHandle(MemoryLayout layout, String name) {
        VarHandle raw = layout.varHandle(PathElement.groupElement(name));
        return MethodHandles.insertCoordinates(raw, 1, 0L);
    }

    public static MemorySegment makeCallback(MethodHandle cbHandle, FunctionDescriptor cbDes) {
        return makeCallback(Arena.ofAuto(), cbHandle, cbDes);
    }

    public static MemorySegment makeCallback(Arena arena, MethodHandle cbHandle, FunctionDescriptor cbDes) {
        return LINKER.upcallStub(cbHandle, cbDes, arena);
    }

    public static UUID makeUUID(MemorySegment segment) {
        assert segment.byteSize() == 16 : "data must be 16 bytes in length";
        long msb = 0;
        long lsb = 0;
        for (int i=0; i<8; i++)
            msb = (msb << 8) | (getByte(segment, i) & 0xff);
        for (int i=8; i<16; i++)
            lsb = (lsb << 8) | (getByte(segment, i) & 0xff);
        return new UUID(msb, lsb);
    }

    public static byte getByte(MemorySegment segment, int index) {
        return segment.get(ValueLayout.JAVA_BYTE, index);
    }
}
