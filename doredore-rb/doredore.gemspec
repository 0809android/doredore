# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = 'doredore'
  spec.version = '0.1.0'
  spec.authors = ['doredore Contributors']
  spec.email = ['your-email@example.com']

  spec.summary = 'Lightweight RAG library with admin UI - Ruby bindings'
  spec.description = 'Fast and simple Retrieval-Augmented Generation (RAG) library built with Rust, providing Ruby bindings via FFI'
  spec.homepage = 'https://github.com/yourusername/rag-enricher'
  spec.license = 'MIT'
  spec.required_ruby_version = '>= 2.7.0'

  spec.metadata['homepage_uri'] = spec.homepage
  spec.metadata['source_code_uri'] = spec.homepage
  spec.metadata['changelog_uri'] = "#{spec.homepage}/blob/main/CHANGELOG.md"

  # Specify which files should be added to the gem
  spec.files = Dir['lib/**/*', 'README.md', 'LICENSE']
  spec.require_paths = ['lib']

  # Runtime dependencies
  spec.add_dependency 'ffi', '~> 1.15'

  # Development dependencies
  spec.add_development_dependency 'rake', '~> 13.0'
  spec.add_development_dependency 'rspec', '~> 3.0'
end
