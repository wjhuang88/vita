package zone.hwj.vita;

import fr.stardustenterprises.yanl.NativeLoader;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.invoke.MethodHandle;

public class NativeCommon {
    private static final Linker LINKER;
    private static final SymbolLookup LOOKUP;

    static {
        new NativeLoader.Builder().build().loadLibrary("vita", false);
        LOOKUP = SymbolLookup.loaderLookup();
        LINKER = Linker.nativeLinker();
    }

    public static MethodHandle makeHandle(String nativeName, FunctionDescriptor descriptor) {
        return LINKER.downcallHandle(LOOKUP.findOrThrow(nativeName), descriptor);
    }

    public static MemorySegment makeCallback(MethodHandle cbHandle, FunctionDescriptor cbDes) {
        return LINKER.upcallStub(cbHandle, cbDes, Arena.ofAuto());
    }
}
