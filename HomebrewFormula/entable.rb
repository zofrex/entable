class Entable < Formula
  desc "Fix mangled padding in tables"
  homepage "https://github.com/zofrex/entable"
  url "https://github.com/zofrex/entable/archive/v0.1.0.tar.gz"
  sha256 "6df8e67cf59c54463f3610631daf3e4861b40c5ea45bca5764628212722bc72c"

  bottle do
    root_url 'https://github.com/zofrex/entable/releases/download/v0.1.0'
    sha256 "c7ea6e144229463f432f457e355864abdad0f4e1699382d9fd7634e12727d8ed" => :high_sierra
  end

  depends_on "rust" => :build

  def install
    system "cargo", "install", "--root", prefix, "--path", "."
  end

  test do
    require 'open3'

    input_data = <<~EOS
      +-------------+---------+-----------+
      | VM | status | role |
      +-------------+---------+-----------+
      | spider | ok | webserver |
      | grasshopper | stopped | db |
      +-------------+---------+-----------+
    EOS

    expected_output = <<~EOS
      +-------------+---------+-----------+
      | VM          | status  | role      |
      +-------------+---------+-----------+
      | spider      | ok      | webserver |
      | grasshopper | stopped | db        |
      +-------------+---------+-----------+
    EOS

    output, status = Open3.capture2("#{bin}/entable", stdin_data: input_data)

    output == expected_output
  end
end
