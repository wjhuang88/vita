package zone.hwj.vita;


public class NativeManager {


    private NativeManager() { }

    public static NativeManager getInstance() {
        final class Holder {
            private static final NativeManager INSTANCE = new NativeManager();
        }

        return Holder.INSTANCE;
    }
}
