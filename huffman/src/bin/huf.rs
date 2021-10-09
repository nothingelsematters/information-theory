use huffman::byte_processor::ByteProcessor;

fn main() {
    huffman::utils::main(&huffman::encode::Encoder::write_processed);
}
