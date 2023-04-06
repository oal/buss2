import uniqolor from 'uniqolor';

export const busColorStyle = (routeNumber: string) => {
  const routeNumberDigits = routeNumber.replace(/\D/g, '');

  const generated = uniqolor(
    `${routeNumber}${Number(routeNumberDigits) * 3.45196392}`, // Avoid similar numbers getting similar colors.
    {
      saturation: [70, 90],
      lightness: [15, 35],
    }
  );
  return {
    backgroundColor: generated.color,
    color: generated.isLight ? '#000' : '#fff',
  };
};

export const parseTimeOrNull = (time: string | null) =>
  time ? new Date(time) : null;
