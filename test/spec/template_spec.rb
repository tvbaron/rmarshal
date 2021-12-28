require_relative '../app_helper'

describe 'template' do
  context = AppHelper.new_context('template')

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
      AppHelper.exec_prog ["#{context.datadir}/input01.json", "--template", "#{context.datadir}/template01.txt", "#{AppHelper.tmpdir}/out.txt"]
      expect(File.read("#{AppHelper.tmpdir}/out.txt")).to eq(File.read("#{context.datadir}/expect01.txt"))
    end
  end
end
