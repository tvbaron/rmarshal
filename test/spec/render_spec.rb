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
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--render", "#{context.datadir}/template01.txt", "--any", "-"], :stdout => sb
      expect(sb.string).to eq(File.read("#{context.datadir}/expect01.txt"))
    end
  end
end
