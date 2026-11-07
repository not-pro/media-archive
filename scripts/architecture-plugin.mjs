const LAYERS = {
  domain: ['application', 'infrastructure', 'presentation', 'platform', 'renderer'],
  application: ['infrastructure', 'presentation', 'platform', 'renderer'],
  infrastructure: ['presentation', 'platform', 'renderer'],
  presentation: ['infrastructure', 'platform', 'renderer'],
  renderer: ['domain', 'application', 'infrastructure', 'presentation', 'platform'],
};

function layerOf(filename) {
  const normalized = filename.replaceAll('\\', '/');
  return (
    Object.keys(LAYERS).find((layer) => normalized.includes(`/src/${layer}/`)) ??
    (normalized.includes('/renderer/') ? 'renderer' : null)
  );
}

function includesLayer(source, layer) {
  return new RegExp(`(?:^|/)${layer}(?:/|$)`).test(source.replaceAll('\\', '/'));
}

function violation(filename, source) {
  const layer = layerOf(filename);
  if (!layer) return null;
  if (source === 'electron' || (layer !== 'infrastructure' && source.startsWith('node:'))) {
    return `${layer} must not depend on ${source}`;
  }
  const forbidden = LAYERS[layer].find((candidate) => includesLayer(source, candidate));
  return forbidden ? `${layer} must not depend on ${forbidden}` : null;
}

function sourceOf(node) {
  if (node.source?.type === 'Literal' && typeof node.source.value === 'string') {
    return node.source.value;
  }
  return null;
}

export default {
  meta: { name: 'local-architecture' },
  rules: {
    'clean-viper': {
      meta: {
        type: 'problem',
        docs: { description: 'Enforce dependency direction and VIPER boundaries' },
        schema: [],
      },
      create(context) {
        function check(node) {
          const source = sourceOf(node);
          if (!source) return;
          const message = violation(context.filename, source);
          if (message) context.report({ node, message });
        }
        return {
          ImportDeclaration: check,
          ImportExpression: check,
          ExportNamedDeclaration: check,
          ExportAllDeclaration: check,
        };
      },
    },
  },
};
