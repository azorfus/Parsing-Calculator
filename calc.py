running = True

types = ["int", "add_sub", "mul", "div"]

class Token:
	typ = ""
	value = ""

class Node:
	term = False
	op = None
	l_value = None
	r_value = None

def lex(S):
	buffer = ""
	i = 0
	digit = False
	d_end = False
	tokens = []
	while i < len(S):
		if S[i].isdigit():
			buffer = buffer + S[i]
			digit = True
			d_end = False
			
			if i == len(S) - 1:
				tok = Token()
				tok.typ = "int"
				tok.value = buffer
				tokens.append(tok)
				buffer = ""
		
		else:
			if digit == True:
				if S[i] == '.':
					d_end = True
					buffer = buffer + '.'
				else:
					tok = Token()
					tok.typ = "int"
					tok.value = buffer
					tokens.append(tok)
					buffer = ""
					digit = False
		
		if S[i] == '+':
			tok = Token()
			tok.typ = "add_sub"
			tok.value = "+"
			tokens.append(tok)

		elif S[i] == '-':
			tok = Token()
			tok.typ = "add_sub"
			tok.value = "-"
			tokens.append(tok)

		elif S[i] == '*':
			tok = Token()
			tok.typ = "mul"
			tok.value = "*"
			tokens.append(tok)

		elif S[i] == '/':
			tok = Token()
			tok.typ = "div"
			tok.value = "/"
			tokens.append(tok)

		i = i + 1
	return tokens

def parse(index, tokens):
	node = Node()
	if index + 2 > len(tokens):
		if tokens[len(tokens) - 1].typ == "int":
			node.term = True
			node.l_value = float(tokens[len(tokens) - 1].value)
		else:
			node.term = True
			node.l_value = 0
	else:
		node.l_value = tokens[index]
		node.op = tokens[index + 1]
		node.r_value = parse(index + 2, tokens)

	return node

def fix_AST(node):
	fixed_AST = Node()
	if node.op == "add_sub" and node.r_value.term == False:
		fixed_AST.l_value = node.l_value
		fixed_AST.op = node.op
		fixed_AST.r_value = fix_AST(node.r_value)
	
	elif node.op == "mul" and node.r_value.term == False:
		if node.r_value.op.typ == "add_sub":
			temp = node.r_value
			node.r_value = node.r_value.l_value
			fixed_AST.l_value = node
			if node.r_value.op.value == "+":
				fixed_AST.op.value = "+"
			else:
				fixed_AST.op.value = "-"
			fixed_AST.r_value = fix_AST(temp.r_value)

	elif node.op == "div" and node.r_value.term == False:
		if node.r_value.op.typ == "add_sub":
			temp = node.r_value
			node.r_value = node.r_value.l_value
			fixed_AST.l_value = node
			if node.r_value.op.value == "+":
				fixed_AST.op.value = "+"
			else:
				fixed_AST.op.value = "-"
			fixed_AST.r_value = fix_AST(temp.r_value)

		elif node.r_value.op.typ == "mul":
			temp = node.r_value
			node.r_value = node.r_value.l_value
			fixed_AST.l_value = node
			fixed_AST.op.value = "*"
			fixed_AST.r_value = fix_AST(temp.r_value)

	else:
		fixed_AST = Node()

	return fixed_AST

while running:
	inp = str(input(">> "))
	if inp == "quit":
		running = False
		break

	tokens = lex(inp)
	for i in tokens:
		print(i.value, end = " ")
	print()
	AST = parse(0, tokens)
	AST = fix_AST(AST)

	# Evaluation of AST here!!!
	# We take care of operator precedence when evaluating
	# instead of taking care of them while parsing.
	# Say we have the nodes for the expression 3*5 + 4
	#      *
	#    3   +
	#      5   4
	# We evaluate it properly by comparing operator precedence
	# then we shift around to get
	#      +
	#    *   4
	#  3   5  
