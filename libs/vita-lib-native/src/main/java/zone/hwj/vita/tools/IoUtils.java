package zone.hwj.vita.tools;

import java.io.ByteArrayOutputStream;
import java.io.FileInputStream;
import java.io.IOException;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;
import java.nio.channels.Channels;
import java.nio.channels.FileChannel;
import java.nio.channels.WritableByteChannel;
import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;

public final class IoUtils {
    private static final int BUFFER_SIZE = 1024 * 8;

    public static byte[] toByteArray(InputStream is) throws IOException {
        try (ByteArrayOutputStream output = new ByteArrayOutputStream()) {
            byte[] b = new byte[BUFFER_SIZE];
            int n;
            while ((n = is.read(b)) != -1) {
                output.write(b, 0, n);
            }
            return output.toByteArray();
        }
    }

    public static String toString(InputStream is, Charset charset) throws IOException {
        return new String(toByteArray(is), charset);
    }

    public static String toUtf8String(InputStream is) throws IOException {
        return toString(is, StandardCharsets.UTF_8);
    }

    public static long copy(InputStream in, OutputStream out, long readLimit) throws IOException {
        if (in instanceof FileInputStream) {
            final FileChannel fc = ((FileInputStream) in).getChannel();
            return channelTransfer(fc, out, readLimit);
        }

        byte[] buf = new byte[BUFFER_SIZE];
        long count = 0;
        int n;
        while ((n = in.read(buf)) > -1) {
            final long nextSize = count + n;
            final long fill;
            if (readLimit == -1 || nextSize < readLimit) {
                fill = n;
            } else {
                fill = readLimit - count;
                if (fill <= 0) {
                    break;
                }
            }
            out.write(buf, 0, (int)fill);
            count += fill;
        }
        return count;
    }

    public static long copyAndFlush(InputStream in, OutputStream out, long readLimit) throws IOException {
        long copied = copy(in, out, readLimit);
        out.flush();
        return copied;
    }

    public static long copy(InputStream in, OutputStream out) throws IOException {
        return copy(in, out, Long.MAX_VALUE);
    }

    public static long copyAndFlush(InputStream in, OutputStream out) throws IOException {
        long copied = copy(in, out);
        out.flush();
        return copied;
    }

    public static void write(OutputStream out, byte[] content) throws IOException {
        if (content == null || out == null || content.length == 0) {
            return;
        }
        out.write(content);
    }

    public static void writeAndFlush(OutputStream out, byte[] content) throws IOException {
        write(out, content);
        out.flush();
    }

    public static void write(OutputStream out, byte[] content, int off, long len) throws IOException {
        if (content == null || out == null || content.length == 0) {
            return;
        }
        out.write(content, off, (int)len);
    }

    public static void writeAndFlush(OutputStream out, byte[] content, int off, long len) throws IOException {
        write(out, content, off, len);
        out.flush();
    }

    public static void write(OutputStream out, CharSequence content) throws IOException {
        if (content == null || out == null || content.isEmpty()) {
            return;
        }
        out.write(content.toString().getBytes(StandardCharsets.UTF_8));
    }

    public static void writeAndFlush(OutputStream out, CharSequence content) throws IOException {
        write(out, content);
        out.flush();
    }

    public static void write(OutputStream out, ByteBuffer content) throws IOException {
        if (content == null || out == null || content.limit() == 0) {
            return;
        }
        int lim = content.limit();
        int cap = content.capacity();
        if (content.hasArray() && lim == cap) {
            out.write(content.array());
        } else {
            while (content.hasRemaining()) {
                out.write(content.get());
            }
        }
    }

    public static void writeAndFlush(OutputStream out, ByteBuffer content) throws IOException {
        write(out, content);
        out.flush();
    }

    private static long channelTransfer(FileChannel fc, OutputStream os, long readLimit) throws IOException {
        WritableByteChannel outChannel = Channels.newChannel(os);
        long size = fc.size();
        if (readLimit != -1 && size >= readLimit) {
            size = readLimit;
        }
        long position = 0;
        while (size > position) {
            long count = fc.transferTo(position, size - position, outChannel);
            if (count > 0) {
                position += count;
            }
        }
        return position;
    }
}
