export type Operator = {
    displayText: string;
    inputText: string;
    name: string;
};

export const OPERATORS: Operator[] = [
    {
        displayText: '¬',
        inputText: '!',
        name: 'Not'
    },
    {
        displayText: '∧',
        inputText: '&',
        name: 'And'
    },
    {
        displayText: '∨',
        inputText: '|',
        name: 'Or'
    },
    {
        displayText: '→',
        inputText: '^',
        name: 'If...then'
    },
    {
        displayText: '↔',
        inputText: '~',
        name: 'If and only if'
    }
];

export function toPreview(rawText: string) {
    let previewText = rawText;
    OPERATORS.forEach((operator) => {
        previewText = previewText.replaceAll(operator.inputText, operator.displayText);
    });
    return previewText;
}