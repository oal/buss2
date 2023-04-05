import uniqolor from 'uniqolor';

export const busColorStyle = (routeNumber: string) => {
  const generated = uniqolor(routeNumber, {
    saturation: [50, 90],
    lightness: 35,
  });
  return {
    backgroundColor: generated.color,
    color: generated.isLight ? '#000' : '#fff',
  };
};
