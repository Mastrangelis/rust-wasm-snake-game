const randomRangeIdx = (minRange: number, maxRange: number) => {
    return Math.floor(Math.random() * (maxRange - minRange + 1)) + minRange;
};

export { randomRangeIdx };
