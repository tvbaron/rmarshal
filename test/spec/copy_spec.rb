describe 'copy JSON file to JSON file' do
  before_all do
    basedir = "#{get :basedir}"
    set :data_dir, "#{basedir}/copy"
    set :tmp_dir, "#{basedir}/tmp"
    make_dir "#{get :tmp_dir}"
  end

  before_each do
    clear_dir "#{get :tmp_dir}"
  end

  after_all do
    clear_dir "#{get :tmp_dir}"
  end

  it 'copies (default)' do
    exec_prog ["#{get :data_dir}/input01.json", "--copy", "#{get :tmp_dir}/out.json"]
    assert { File.read("#{get :tmp_dir}/out.json") == File.read("#{get :data_dir}/expect01.json") }
  end

  it 'copies (pretty)' do
    exec_prog ["#{get :data_dir}/input01.json", "--copy", "--json", "--pretty", "#{get :tmp_dir}/out.json"]
    assert { File.read("#{get :tmp_dir}/out.json") == File.read("#{get :data_dir}/expect02.json") }
  end
end

describe 'copy JSON file to JSON stdout' do
  before_all do
    basedir = "#{get :basedir}"
    set :data_dir, "#{basedir}/copy"
    set :tmp_dir, "#{basedir}/tmp"
    make_dir "#{get :tmp_dir}"
  end

  before_each do
    clear_dir "#{get :tmp_dir}"
  end

  after_all do
    clear_dir "#{get :tmp_dir}"
  end

  it 'copies (default)' do
    sb = StringIO.new
    exec_prog ["#{get :data_dir}/input01.json", "--copy", "--json", "-"], :stdout => sb
    assert { sb.string == File.read("#{get :data_dir}/expect01.json") }
  end

  it 'copies (pretty)' do
    sb = StringIO.new
    exec_prog ["#{get :data_dir}/input01.json", "--copy", "--json", "--pretty", "-"], :stdout => sb
    assert { sb.string == File.read("#{get :data_dir}/expect02.json") }
  end
end

describe 'copy JSON stdin to JSON file' do
  before_all do
    basedir = "#{get :basedir}"
    set :data_dir, "#{basedir}/copy"
    set :tmp_dir, "#{basedir}/tmp"
    make_dir "#{get :tmp_dir}"
  end

  before_each do
    clear_dir "#{get :tmp_dir}"
  end

  after_all do
    clear_dir "#{get :tmp_dir}"
  end

  it 'copies (default)' do
    sb = StringIO.new
    sb.puts '{"name":"Althea"}'
    exec_prog ["--json", "-", "--copy", "#{get :tmp_dir}/out.json"], :stdin => sb
    assert { File.read("#{get :tmp_dir}/out.json") == File.read("#{get :data_dir}/expect01.json") }
  end

  it 'copies (pretty)' do
    sb = StringIO.new
    sb.puts '{"name":"Althea"}'
    exec_prog ["--json", "-", "--copy", "--json", "--pretty", "#{get :tmp_dir}/out.json"], :stdin => sb
    assert { File.read("#{get :tmp_dir}/out.json") == File.read("#{get :data_dir}/expect02.json") }
  end
end

describe 'copy JSON stdin to JSON stdout' do
  before_all do
    basedir = "#{get :basedir}"
    set :data_dir, "#{basedir}/copy"
    set :tmp_dir, "#{basedir}/tmp"
    make_dir "#{get :tmp_dir}"
  end

  before_each do
    clear_dir "#{get :tmp_dir}"
  end

  after_all do
    clear_dir "#{get :tmp_dir}"
  end

  it 'copies (default)' do
    si = StringIO.new
    si.puts '{"name":"Althea"}'
    so = StringIO.new
    exec_prog ["--json", "-", "--copy", "--json", "-"], :stdin => si, :stdout => so
    assert { so.string == File.read("#{get :data_dir}/expect01.json") }
  end

  it 'copies (pretty)' do
    si = StringIO.new
    si.puts '{"name":"Althea"}'
    so = StringIO.new
    exec_prog ["--json", "-", "--copy", "--json", "--pretty", "-"], :stdin => si, :stdout => so
    assert { so.string == File.read("#{get :data_dir}/expect02.json") }
  end
end
