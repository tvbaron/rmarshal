require_relative '../app_helper'

describe 'render' do
  context = AppHelper.new_context('render')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'render a template to a file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'renders' do
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template01.txt", "#{AppHelper.tmpdir}/out.txt"]
      expect(File.read("#{AppHelper.tmpdir}/out.txt")).to eq(File.read("#{context.datadir}/expect01.txt"))
    end
  end

  describe 'render a template to stdout' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'renders' do
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template01.txt", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect01.txt"))
    end
  end

  describe 'render a template with comment' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'renders with trailing \'%>\'' do
      expect = <<EOS
===begin===

===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template02.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end

    it 'renders with trailing \'-%>\'' do
      expect = <<EOS
===begin===
===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template03.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end
  end

  describe 'render a template with statement line' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'renders with heading \'%\'' do
      expect = <<EOS
===begin===
My name is Althea.
And my score is 99.
% Hey!
===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template04.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end
  end

  describe 'render a template with statement' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'renders with heading \'<%\'' do
      expect = <<EOS
===begin===
  |
My name is Althea.
And my score is 99.
===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template05.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end

    it 'renders with heading \'<%-\'' do
      expect = <<EOS
===begin===
|
My name is Althea.
And my score is 99.
===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template06.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end

    it 'renders with heading \'<%-\' and trailing \'-%>\'' do
      expect = <<EOS
===begin===
My name is Althea.
And my score is 99.
===end===
EOS
      sb = StringIO.new
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template07.txt", "-"], :stdout => sb
      expect(sb.string).to eq(expect)
    end
  end
end
