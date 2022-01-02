require_relative '../app_helper'

describe 'merge' do
  context = AppHelper.new_context('merge')

  before :all do
    AppHelper.make_dir(AppHelper.tmpdir)
  end

  after :all do
    AppHelper.clear_dir(AppHelper.tmpdir)
  end

  describe 'merge to JSON file' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'merges' do
      AppHelper.exec_prog ["#{context.datadir}/input01.yaml", "#{context.datadir}/input02.yaml", "--merge", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect01.json"))
    end
  end

  describe 'merge to JSON file (object with depth)' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'merges (default)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'merges (depth -1)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "-1", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'merges (depth 0)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "0", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect03.json"))
    end

    it 'merges (depth 1)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "1", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect04.json"))
    end

    it 'merges (depth 2)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "2", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect05.json"))
    end

    it 'merges (depth 3)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "3", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end

    it 'merges (depth 4)' do
      AppHelper.exec_prog ["#{context.datadir}/input03.yaml", "#{context.datadir}/input04.yaml", "--merge", "--depth", "4", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect02.json"))
    end
  end

  describe 'merge to JSON file (array with depth)' do
    before :each do
      AppHelper.clear_dir(AppHelper.tmpdir)
    end

    it 'merges (default)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect06.json"))
    end

    it 'merges (depth -1)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "-1", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect06.json"))
    end

    it 'merges (depth 0)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "0", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect07.json"))
    end

    it 'merges (depth 1)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "1", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect08.json"))
    end

    it 'merges (depth 2)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "2", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect09.json"))
    end

    it 'merges (depth 3)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "3", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect10.json"))
    end

    it 'merges (depth 4)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "4", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect11.json"))
    end

    it 'merges (depth 5)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "5", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect12.json"))
    end

    it 'merges (depth 6)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "6", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect06.json"))
    end

    it 'merges (depth 7)' do
      AppHelper.exec_prog ["#{context.datadir}/input05.yaml", "#{context.datadir}/input06.yaml", "--merge", "--depth", "7", "--json", "--pretty", "#{AppHelper.tmpdir}/out.json"]
      expect(File.read("#{AppHelper.tmpdir}/out.json")).to eq(File.read("#{context.datadir}/expect06.json"))
    end
  end
end
