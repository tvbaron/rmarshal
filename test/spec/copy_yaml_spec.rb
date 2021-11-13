describe 'copy YAML file to YAML file' do
    before_all do
      basedir = "#{get :basedir}"
      set :data_dir, "#{basedir}/copy_yaml"
      set :tmp_dir, "#{basedir}/tmp"
      make_dir "#{get :tmp_dir}"
    end

    before_each do
      clear_dir "#{get :tmp_dir}"
    end

    after_all do
      clear_dir "#{get :tmp_dir}"
    end

    it 'copies' do
      exec_prog ["#{get :data_dir}/input01.yaml", "--copy", "#{get :tmp_dir}/out.yaml"]
      assert { File.read("#{get :tmp_dir}/out.yaml") == File.read("#{get :data_dir}/expect01.yaml") }
    end
  end
