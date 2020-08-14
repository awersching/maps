const roadTypeColors = {
  Motorway: '#264653',
  Trunk: '#2a9d8f',
  Primary: '#e9c46a',
  Secondary: '#f4a261',
  Tertiary: '#e76f51',
  Unclassified: '#e63946',
  Residential: '#f1faee',
  LivingStreet: '#a8dadc',
  Service: '#457b9d',
  Pedestrian: '#8338ec',
  Track: '#3a86ff',
  Road: '#99582a',
  Footway: '#979dac',
  Steps: '#fbff12',
  Path: '#ffa5ab',
  Cycleway: '#276321',
};

const roadSurfaceColors = {
  Paved: '#264653',
  Unpaved: '#2a9d8f',
  Asphalt: '#979dac',
  Concrete: '#e9c46a',
  PavingStones: '#ffa5ab',
  Sett: '#457b9d',
  Cobblestone: '#f1faee',
  Metal: '#a8dadc',
  Wood: '#e63946',
  Compacted: '#8338ec',
  FineGravel: '#6d7980',
  Gravel: '#953272',
  Pebblestone: '#3a86ff',
  Plastic: '#f4a261',
  GrassPaver: '#7fb800',
  Grass: '#276321',
  Dirt: '#99582a',
  Earth: '#54211C',
  Mud: '#e76f51',
  Sand: '#fbff12',
  Ground: '#ff5400',
};

export function roadTypeColor(roadType) {
  return roadTypeColors[roadType] ? roadTypeColors[roadType] : '#000000';
}

export function roadSurfaceColor(roadSurface) {
  return roadSurfaceColors[roadSurface] ? roadSurfaceColors[roadSurface] : '#000000';
}
