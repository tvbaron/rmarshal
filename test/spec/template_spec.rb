describe 'render a template to a file' do
  before_all do
    basedir = "#{get :basedir}"
    set :data_dir, "#{basedir}/template"
    set :tmp_dir, "#{basedir}/tmp"
    make_dir "#{get :tmp_dir}"
  end

  before_each do
    clear_dir "#{get :tmp_dir}"
  end

  after_all do
    clear_dir "#{get :tmp_dir}"
  end

  it 'renders' do
    exec_prog ["#{get :data_dir}/input01.json", "--template", "#{get :data_dir}/template01.txt", "#{get :tmp_dir}/out.txt"]
    assert { File.read("#{get :tmp_dir}/out.txt") == File.read("#{get :data_dir}/expect01.txt") }
  end
end
